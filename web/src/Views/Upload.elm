module Views.Upload exposing (load, update, Msg, Model, render, subscriptions)

import Files
import Html exposing (Html, div, h1, text, input, form, button)
import Html.Attributes exposing (type_, multiple, classList)
import Html.Events
import UI.Components as Components
import AddressableStates
import FileReader exposing (NativeFile, parseSelectedFiles)
import Json.Decode as Json exposing (Value, andThen)
import Pure
import Http.Progress
import Task
import Array

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

type Msg
  = SelectFiles (List NativeFile)
  | UploadFiles (List NativeFile)
  | UploadProgress (Http.Progress.Progress Service.UploadResult)
  | StartUpload NativeFile

type ViewState
  = NoSelection
  | Selected (List NativeFile)
  | Uploading (Array.Array NativeFile)

type alias Model =
  { targetDir: Files.FilePath
  , state: ViewState
  }

load : Files.FilePath -> (Model, Cmd Msg)
load path = ( { targetDir = path, state = NoSelection }, Cmd.none )

update : Model -> Msg -> (Model, Cmd Msg)
update model msg =
  case msg of
    SelectFiles x -> ( { model | state = Selected x }, Cmd.none )
    UploadFiles x -> ( { model | state = Uploading (List.head x, Maybe.withDefault [] (List.tail x)) }, Cmd.none )

    UploadProgress (Http.Progress.Done x) ->
      case model.state of
        Uploading (cur, toUpload) -> ( { model | state = Uploading (List.head toUpload, Maybe.withDefault [] (List.tail toUpload)) }, Cmd.none )
        _ -> (model, Cmd.none)

    UploadProgress (Http.Progress.Some x) ->
      let
        _ = Debug.log(toString x.bytes)
      in
        (model, Cmd.none)

    UploadProgress (Http.Progress.Fail x) ->
      case model.state of
        Uploading (cur, toUpload) ->
          let
            _ = Debug.log ("Failed to upload ")
          in
            ( { model | state = Uploading (List.head toUpload, Maybe.withDefault [] (List.tail toUpload)) }, Cmd.none )
        _ -> (model, Cmd.none)

    UploadProgress _ -> Debug.crash("nothing yet")

subscriptions : Model -> Sub Msg
subscriptions model =
  case model.state of
    Uploading (curFile, files) ->
      case curFile of
        Just nativeFile ->
          Service.uploadFile "testing" nativeFile |>
            Http.Progress.track "/add-file" UploadProgress
        _ -> Sub.none

    _ -> Sub.none

onchange : (List NativeFile -> value) -> Html.Attribute value
onchange action = Html.Events.on "change" (Json.map action parseSelectedFiles)

renderFile : NativeFile -> Html Msg
renderFile file =
  div [] [text file.name]

renderFileStatus : List NativeFile -> Html Msg
renderFileStatus selectedFiles =
  div [] (List.map renderFile selectedFiles)

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
      [ h1 [] [ text "Multiple files with automatic upload" ]
      , input
        [ type_ "file"
        , onchange SelectFiles
        , multiple True
        ]
        []
      ] ++ extraContent
  in
    div [] content


render : Model -> Html Msg
render model =
  let
    content = case model.state of
      NoSelection -> renderUploadForm model
      Selected x -> renderUploadForm model
      Uploading (curIndex, x) -> renderFileStatus x
  in
    div []
      [ Components.closeableHeader "Upload File" <| AddressableStates.generateFolderAddress model.targetDir
      , content
      ]
