module Views.Upload exposing (load, update, Msg, Model, render)

import Files
import Html exposing (Html, div, h1, text, input)
import Html.Attributes exposing (type_, multiple)
import Html.Events
import UI.Components as Components
import AddressableStates
import FileReader exposing (NativeFile, parseSelectedFiles)
import Json.Decode as Json exposing (Value, andThen)

type Msg
  = UploadFiles (List NativeFile)

type alias Model =
  { targetDir: Files.FilePath
  , selectedFiles: List NativeFile
  }

load : Files.FilePath -> (Model, Cmd Msg)
load path = ( { targetDir = path, selectedFiles = [] }, Cmd.none )

update : Model -> Msg -> (Model, Cmd Msg)
update model msg =
  case msg of
    UploadFiles x -> ( { model | selectedFiles = x }, Cmd.none )

onchange : (List NativeFile -> value) -> Html.Attribute value
onchange action = Html.Events.on "change" (Json.map action parseSelectedFiles)

renderFile : NativeFile -> Html Msg
renderFile file =
  div [] [text file.name]

renderFileStatus : Model -> Html Msg
renderFileStatus model =
  div [] (List.map renderFile model.selectedFiles)

renderUploadForm : Model -> Html Msg
renderUploadForm model =
  div []
     [ h1 [] [ text "Multiple files with automatic upload" ]
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
    content = case model.selectedFiles of
      [] -> renderUploadForm model
      _ -> renderFileStatus model
  in
    div []
      [ Components.closeableHeader "Upload File" <| AddressableStates.generateFolderAddress model.targetDir
      , content
      ]
