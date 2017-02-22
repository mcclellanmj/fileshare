module Views.Upload exposing (load, update, Msg, Model, render)

import Files
import Html exposing (Html, div, h1, text, input)
import Html.Attributes exposing (type_, multiple)
import Html.Events
import UI.Components as Components
import AddressableStates
import FileReader exposing (NativeFile, parseSelectedFiles)
import Json.Decode as Json exposing (Value, andThen)
import Http.Progress
import Task
import Array
import List.Extra

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
  { curIndex: Maybe Int
  , files: Array.Array FileUpload
  }

type Msg
  = SelectFiles (List NativeFile)
  | UploadFiles (List NativeFile)
  | UploadFile Int
  | UploadProgress Int (Http.Progress.Progress Service.UploadResult)
  | StartUpload NativeFile

type ViewState
  = NoSelection
  | Selected (List NativeFile)
  | Uploading UploadingState
  | FinishedUploading (List UploadingState)

type alias Model =
  { targetDir: Files.FilePath
  , selectedFiles: List NativeFile
  }

load : Files.FilePath -> (Model, Cmd Msg)
load path = ( { targetDir = path, selectedFiles = [] }, Cmd.none )

nativeToFileUpload : NativeFile -> FileUpload
nativeToFileUpload f =
  { file = f
  , status = NotStarted
  }

getNextUpload : Array.Array FileUpload -> Maybe Int
getNextUpload files =
  List.Extra.findIndex (\x -> x.status == NotStarted) (Array.toList files)

handleFailed : UploadingState -> ViewState
handleFailed uploadingState =
  case getNextUpload uploadingState.files of
    Nothing -> FinishedUploading (Array.toList uploadingState.files)
    Just x -> Uploading { curIndex = x, files = uploadingState.files }

createInitialUploading : List NativeFile -> UploadingState
createInitialUploading allFiles =
  let
    filesWithStatus = Array.fromList (List.map nativeToFileUpload allFiles)
  in
    { curIndex = getNextUpload filesWithStatus
    , files = filesWithStatus
    }

update : Model -> Msg -> (Model, Cmd Msg)
update model msg =
  case msg of
    SelectFiles x -> ( { model | state = Selected x }, Cmd.none )
    UploadFiles x -> ( { model | state = Uploading (createInitialUploading x) }, Cmd.none )

    UploadProgress idx (Http.Progress.Done x) ->
      case model.state of
        Uploading uploadingState -> ( model, Cmd.none )
        _ -> ( model, Cmd.none )

    UploadProgress idx (Http.Progress.Some x) ->
      let
        _ = Debug.log(toString x.bytes)
      in
        (model, Cmd.none)

    UploadProgress idx (Http.Progress.Fail x) ->
      case model.state of
        Uploading uploadingState ->
          ( { model | state = Uploading (List.head toUpload, Maybe.withDefault [] (List.tail toUpload)) }, Cmd.none )
        _ -> (model, Cmd.none)

    UploadProgress _ _ -> Debug.crash("nothing yet")

subscriptions : Model -> Sub Msg
subscriptions model =
  case model.state of
    Uploading uploadStatus ->
      case uploadStatus.curIndex of
        Just i ->
          case Array.get i uploadStatus.files of
            Just file -> Service.uploadFile "testing" file.file |> Http.Progress.track "/add-file" (UploadProgress i)
            Nothing -> Debug.crash "Tried to get an index which does not exist"
        Nothing -> Sub.none
    _ -> Sub.none

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
        [ button
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

render : Model -> Html Msg
render model =
  let
    content = case model.state of
      NoSelection -> renderUploadForm model
      Selected x -> renderUploadForm model
      Uploading uploadStatus -> renderFileStatus uploadStatus
  in
    div []
      [ Components.closeableHeader "Upload File" <| AddressableStates.generateFolderAddress model.targetDir
      , content
      ]
