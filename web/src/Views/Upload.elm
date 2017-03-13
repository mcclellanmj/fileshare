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
import Task

import Service

type UploadStatus
  = NotStarted
  | InProgress Int
  | Finished
  | Failed

type alias FileUpload =
  { file: NativeFile
  , status: UploadStatus
  }

type alias UploadingState =
  { curIndex: Int
  , files: Array.Array FileUpload
  }

type Msg
  = SelectFiles (List NativeFile)
  | UploadFiles (List NativeFile)
  | UploadProgress Int (Http.Progress.Progress Service.UploadResult)
  | FinishedFile Int

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
markIndexFail index files =
  let
    fileAtIndex = Array.get index files
    updatedFiles =
      case fileAtIndex of
        Just x -> Just { x | status = Failed}
        Nothing -> Nothing
  in
    case updatedFiles of
      Just x -> Array.set index x files
      Nothing -> Debug.crash "Was not able to find file at index"

toNextUploadState : UploadingState -> ViewState
toNextUploadState currentState =
  case getNextUpload currentState.files of
    Just nextIdx -> Uploading { currentState | curIndex = nextIdx }
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

    UploadProgress idx (Http.Progress.Done x) -> Debug.crash ("Done progress not implemented yet")
    UploadProgress idx (Http.Progress.Some x) -> Debug.crash ("Some prgress not implemented yet")

    UploadProgress idx (Http.Progress.Fail x) ->
      case model.state of
        Uploading uploadingState ->
          ( { model | state = Uploading (handleFailedUpload uploadingState) }, Task.perform (FinishedFile) (Task.succeed idx))
        _ -> Debug.crash "Upload progress failed but got somewhere weird"

    FinishedFile idx ->
      case model.state of
        Uploading uploadingState ->
          ( {model | state = toNextUploadState uploadingState }, Cmd.none )
        _ -> Debug.crash "Finished a file but not in the uploading state"

    UploadProgress idx (Http.Progress.None) -> Debug.crash "No Progress not implemented yet"

subscriptions : Model -> Sub Msg
subscriptions model =
  case model.state of
    Uploading uploadStatus ->
      case Array.get uploadStatus.curIndex uploadStatus.files of
        Just file ->
          let
            nextUpload = Debug.log "Next sub upload" file.file
            index = Debug.log "Next index will be" uploadStatus.curIndex
          in
            Debug.log ("Subscribing") (Service.uploadFile nextUpload.name nextUpload |> Http.Progress.track ("/add-file" ++ toString index) (UploadProgress index))
        Nothing -> Debug.crash "Tried to get an index which does not exist"
    _ -> Debug.log ("got no subscriptions") Sub.none

onchange : (List NativeFile -> value) -> Html.Attribute value
onchange action = Html.Events.on "change" (Json.map action parseSelectedFiles)

renderFile : FileUpload -> Html Msg
renderFile upload =
  div [] [text upload.file.name]

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
      FinishedUploading files -> div [] [text "All Done"]
  in
    div []
      [ Components.closeableHeader "Upload File" <| AddressableStates.generateFolderAddress model.targetDir
      , content
      ]
