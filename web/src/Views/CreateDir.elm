module Views.CreateDir exposing (load, update, Msg, Model, render)

import Files
import Html exposing (Html, div, text, h1, input)
import Debug
import Http
import Result.Extra
import Service
import Navigation
import AddressableStates

import Bootstrap.Button as Button
import Bootstrap.Form as Form
import Bootstrap.Form.Input as Input
import Bootstrap.Grid as Grid
import Bootstrap.Grid.Col as Col

type Msg
  = DoCreate String
  | InputDirectoryName String
  | CreateFinished ()
  | CreateFailed Http.Error

type alias Model =
  { targetDir: Files.FilePath
  , directoryName: String
  , errors: Maybe Http.Error
  }

load : Files.FilePath -> (Model, Cmd Msg)
load path = ( { targetDir = path, directoryName = "", errors = Nothing }, Cmd.none )

createCmd: String -> String -> Cmd Msg
createCmd path email = Http.send (Result.Extra.unpack CreateFailed CreateFinished) (Service.createDirectory path email)

update : Model -> Msg -> (Model, Cmd Msg)
update model msg =
  case msg of
    DoCreate dirName -> ( model, createCmd model.targetDir model.directoryName)
    InputDirectoryName dirName -> ( { model | directoryName = dirName }, Cmd.none )
    CreateFinished _ -> ( model, Navigation.newUrl <| AddressableStates.generateFolderAddress model.targetDir)
    CreateFailed error -> Debug.crash "Failed has not been implemented"

render : Model -> Html Msg
render model =
  Grid.row
    []
    [ Grid.col []
      [ h1 [] [ text "Create Directory" ]
      , Form.form []
        [ Form.row []
          [ Form.col [ Col.sm2 ] [ Form.label [] [ text "Name" ]]
          , Form.col [ Col.sm10 ] [ Input.text  [ Input.onInput InputDirectoryName ]]
          ]
        , Form.row []
          [ Form.col []
            [ Button.button
              [ Button.primary
              , Button.onClick (DoCreate model.directoryName)
              ]
              [ text "Create" ]
            ]
          ]
        ]
      ]
    ]
