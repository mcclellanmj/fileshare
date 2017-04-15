module Views.CreateDir exposing (load, update, Msg, Model, render)

import Files
import Html exposing (Html, div, text, h1, input)
import Html.Attributes as Attributes
import Debug
import Html.Events
import Http
import Result.Extra
import Service
import Navigation
import AddressableStates
import Bootstrap.Button as Button

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
  div []
    [ h1 [] [ text "Create Directory" ]
    , input
        [ Attributes.type_ "text"
        , Html.Events.onInput InputDirectoryName]
        []
    , div []
        [ Button.button
          [ Button.primary ]
          [ text "Create Directory" ]
        ]
    ]

