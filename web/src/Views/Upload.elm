module Views.Upload exposing (load, update, Msg, Model, render, subscriptions)

import Files
import Html exposing (Html, div, h1, text, input)
import Html.Attributes exposing (type_, multiple, classList)
import Html.Events
import Pure
import UI.Components as Components
import AddressableStates
import FileReader exposing (NativeFile, parseSelectedFiles)
import Json.Decode as Json exposing (Value, andThen)
import Http.Progress
import Array
import List.Extra

import Service
import Css

type UploadStatus
  = NotStarted
  | InProgress Int
  | Finished
  | Failed

type alias FileUpload =
  { file: NativeFile
  , status: UploadStatus
  }

-- FIXME: See if this can be done with lists instead of Array because array is clunky
type alias UploadingState =
  { curIndex: Int
  , files: Array.Array FileUpload
  }

type Msg
  = SelectFiles (List NativeFile)
  | UploadFiles (List NativeFile)
  | UploadProgress Int (Http.Progress.Progress Service.UploadResult)

type ViewState
  = NoSelection
  | Selected (List NativeFile)
  | Uploading UploadingState
  | FinishedUploading UploadingState

type alias Model =
  { targetDir: Files.FilePath
  , state: ViewState
  }

load : Files.FilePath -> (Model, Cmd Msg)
load path = ( { targetDir = path, state = NoSelection }, Cmd.none )

nativeToFileUpload : NativeFile -> FileUpload
nativeToFileUpload f =
  { file = f
  , status = NotStarted
  }

getNextUpload : Array.Array FileUpload -> Maybe Int
getNextUpload files =
  List.Extra.findIndex (\x -> x.status == NotStarted) (Array.toList files)

markIndexFail : Int -> Array.Array FileUpload -> Array.Array FileUpload
markIndexFail index files = markIndex Failed index files

markIndex: UploadStatus -> Int -> Array.Array FileUpload -> Array.Array FileUpload
markIndex newStatus index files =
  let
    fileAtIndex = Array.get index files
    updatedFile =
      case fileAtIndex of
        Just x -> Just { x | status = newStatus }
        Nothing -> Nothing
  in
    case updatedFile of
      Just x -> Array.set index x files
      Nothing -> Debug.crash "Tried to update an impossible index"

setCurIndexState : UploadingState -> UploadStatus -> UploadingState
setCurIndexState uploadingState newStatus =
  { uploadingState | files = markIndex newStatus uploadingState.curIndex uploadingState.files }

toNextUploadState : UploadingState -> ViewState
toNextUploadState currentState =
  case getNextUpload currentState.files of
    Just nextIdx -> Uploading { currentState | curIndex = nextIdx, files = markIndex (InProgress 0) nextIdx currentState.files }
    Nothing -> FinishedUploading currentState

handleFailedUpload : UploadingState -> UploadingState
handleFailedUpload uploadingState =
  let
    updatedFiles = markIndexFail uploadingState.curIndex uploadingState.files
  in
    { uploadingState | files = updatedFiles }

setIndexInProgress : Int -> Array.Array FileUpload -> Array.Array FileUpload
setIndexInProgress index files =
  let
    updatedFile = case Array.get index files of
      Just x -> { x | status = InProgress 0 }
      Nothing -> Debug.crash "Unable to find file at index"
  in
    Array.set index updatedFile files

addProgress : UploadingState -> { bytes : Int, bytesExpected : Int} -> UploadingState
addProgress uploadingState amount =
  let
    index = uploadingState.curIndex
    amt = Debug.log ("Got progress of " ++ (toString amount.bytes) ++ " on file at index " ++ (toString index)) amount.bytes
    updatedFile = case Array.get index uploadingState.files of
      Just x -> { x | status = InProgress amt }
      Nothing -> Debug.crash ("Unable to find file at index" ++ (toString index))
    updatedArray = Array.set index updatedFile uploadingState.files
  in
    { uploadingState | files = updatedArray }

createInitialUploading : List NativeFile -> UploadingState
createInitialUploading allFiles =
  let
    filesWithStatus = Array.fromList (List.map nativeToFileUpload allFiles)
    curIndex = getNextUpload filesWithStatus
  in
    case curIndex of
      Just x ->
        { curIndex = x
        , files = setIndexInProgress x filesWithStatus
        }
      Nothing -> Debug.crash "Cannot start upload"

update : Model -> Msg -> (Model, Cmd Msg)
update model msg =
  case msg of
    SelectFiles x -> ( { model | state = Selected x }, Cmd.none )
    UploadFiles x -> ( { model | state = Uploading (createInitialUploading x) }, Cmd.none )

    UploadProgress idx (Http.Progress.Done x) ->
      case model.state of
        Uploading uploadingState ->
          let
            markedFinished = setCurIndexState uploadingState Finished
          in
            ( { model | state = toNextUploadState markedFinished }, Cmd.none)
        _ -> Debug.crash "Cannot update state when not in an upload state"

    UploadProgress idx (Http.Progress.Some amount) ->
      case model.state of
        Uploading uploadingState ->
          ( { model | state = Uploading (addProgress uploadingState amount) }, Cmd.none )
        _ -> Debug.crash "Cannot update state when not in an upload state"

    UploadProgress idx (Http.Progress.Fail x) ->
      case model.state of
        Uploading uploadingState ->
          ( { model | state = toNextUploadState <| handleFailedUpload uploadingState }, Cmd.none)
        _ -> Debug.crash "Upload progress failed but got somewhere weird"

    UploadProgress idx (Http.Progress.None) -> Debug.crash "No Progress not implemented yet"

subscriptions : Model -> Sub Msg
subscriptions model =
  case model.state of
    Uploading uploadStatus ->
      case Array.get uploadStatus.curIndex uploadStatus.files of
        Just file ->
          let
            nextUpload = file.file
            index = uploadStatus.curIndex
          in
            Service.uploadFile nextUpload.name nextUpload |> Http.Progress.track ("/add-file" ++ toString index) (UploadProgress index)
        Nothing -> Debug.crash "Tried to get an index which does not exist"
    _ -> Sub.none

onchange : (List NativeFile -> value) -> Html.Attribute value
onchange action = Html.Events.on "change" (Json.map action parseSelectedFiles)

renderFile : FileUpload -> Html Msg
renderFile upload =
  let
    renderFileText: List (Html.Attribute a) -> String -> Html a
    renderFileText attr fileText = div attr [text fileText]
  in
    case upload.status of
      NotStarted -> renderFileText [] upload.file.name
      InProgress x -> renderFileText [Css.withClass Css.FileUploadInProgress] (upload.file.name ++ " " ++ (toString x))
      Failed -> renderFileText [Css.withClass Css.FileUploadFailed] upload.file.name
      Finished -> renderFileText [Css.withClass Css.FileUploadFinished] upload.file.name

renderFileStatus : UploadingState -> Html Msg
renderFileStatus state =
  div []
    (Array.toList (Array.map renderFile state.files))

renderUploadForm : Model -> Html Msg
renderUploadForm model =
  let
    extraContent = case model.state of
      Selected x ->
        [ Html.button
          [ classList [(Pure.buttonPrimary, True)]
          , Html.Events.onClick (UploadFiles x)
          ]
          [text "Upload"]
        ]
      _ -> []

    content =
      [ h1 [] [ text "Upload Files" ]
      , input
        [ type_ "file"
        , onchange UploadFiles
        , multiple True
        ]
        []
      ]
  in
    div [] ( content ++ extraContent )

render : Model -> Html Msg
render model =
  let
    content = case model.state of
      NoSelection -> renderUploadForm model
      Selected x -> renderUploadForm model
      Uploading uploadStatus -> renderFileStatus uploadStatus
      -- FIXME: Should give some indication that it finished
      FinishedUploading uploadStatus -> renderFileStatus uploadStatus
  in
    div []
      [ Components.closeableHeader "Upload File" <| AddressableStates.generateFolderAddress model.targetDir
      , content
      ]
