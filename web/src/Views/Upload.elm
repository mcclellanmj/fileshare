module Views.Upload exposing (load, update, Msg, Model, render)

import Files
import Html exposing (Html)
import Debug

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
render model = Debug.crash "Render is not yet implemented"
