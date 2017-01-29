module Views.Upload exposing (load, update, Msg, Model, render)

import Files
import Html exposing (Html, div)
import Debug
import UI.Components as Components
import AddressableStates

type Msg
  = DoUpload

type alias Model =
  { targetDir: Files.FilePath
  }

load : Files.FilePath -> (Model, Cmd Msg)
load path = ( { targetDir = path }, Cmd.none )

update : Model -> Msg -> (Model, Cmd Msg)
update model msg =
  case msg of
    DoUpload -> Debug.crash "DoUpload is not yet implemented"

render : Model -> Html Msg
render model =
  div
    []
    [ Components.closeableHeader "Upload File" <| AddressableStates.generateFolderAddress model.targetDir ]
