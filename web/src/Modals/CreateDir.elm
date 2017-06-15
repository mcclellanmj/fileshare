module Modals.CreateDir exposing (update, Msg(..), Model, view, initialState)

import Http
import Result.Extra
import Service
import Html exposing (text, Html)
import Task

import Bootstrap.Modal as Modal
import Bootstrap.Form as Form
import Bootstrap.Grid.Col as Col
import Bootstrap.Button as Button
import Bootstrap.Form.Input as Input

type alias Model =
  { modalState: Modal.State
  , createState: CreateState
  , targetDirectory: String
  , directoryName: String
  }

type CreateState
  = Input
  | Success

type Msg
  = DoCreate String
  | InputDirectoryName String
  | CreateFinished ()
  | CreateFailed Http.Error
  | ModalMsg Modal.State
  | ShowModal String

initialState: Model
initialState =
  { modalState = Modal.hiddenState
  , createState = Input
  , targetDirectory = ""
  , directoryName = ""
  }

createCmd: String -> String -> Cmd Msg
createCmd path email = Http.send (Result.Extra.unpack CreateFailed CreateFinished) (Service.createDirectory path email)

update : Model -> Msg -> (Model, Cmd Msg)
update model msg =
  case msg of
    DoCreate dirName -> ( model, createCmd model.targetDirectory dirName)
    InputDirectoryName dirName -> ( { model | directoryName = dirName }, Cmd.none )
    CreateFinished _ -> ( { model | createState = Success }, Cmd.none )
    CreateFailed error -> Debug.crash "Failed has not been implemented"
    ShowModal directory -> ( { initialState | targetDirectory = directory } , Task.succeed (ModalMsg Modal.visibleState) |> Task.perform identity )
    ModalMsg modalState -> ( { model | modalState = modalState }, Cmd.none )

viewForm : Model -> Html Msg
viewForm model =
  Form.form [] [
    Form.row []
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

viewSuccess : Model -> Html Msg
viewSuccess model =
  Html.div [] [ text "Successfully Created!" ]

viewModalBody : Model -> Html Msg
viewModalBody model =
    case model.createState of
      Success -> viewSuccess model
      Input -> viewForm model

view : Model -> Html Msg
view model =
  Modal.config ModalMsg
    |> Modal.h3 [] [ text "Create Directory" ]
    |> Modal.body [] [ viewModalBody model ]
    |> Modal.view model.modalState
