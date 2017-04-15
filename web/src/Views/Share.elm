module Views.Share exposing (render, Model, Msg, update, load)

import Html exposing (Html, div, span, text, a)
import Html.Attributes exposing (style, classList, href)
import Html.Events exposing (onClick, onSubmit)
import Service exposing (File)
import FontAwesome.Web as Icons
import UI.Components as Components
import AddressableStates as States
import Files exposing (FilePath)
import Service exposing (ShareResult)
import Http
import Errors
import Result.Extra

import Bootstrap.Button as Button

type State
  = Input
  | Sharing
  | Finished ShareResult
  | Failed Http.Error

type alias Model =
  { sharing: FilePath
  , return: FilePath
  , state: State
  }

type Msg
  = DoShare String
  | ShareFailed Http.Error
  | ShareFinished ShareResult

shareCmd: String -> String -> Cmd Msg
shareCmd path email = Http.send (Result.Extra.unpack ShareFailed ShareFinished) (Service.shareFile path email)

load : FilePath -> FilePath -> (Model, Cmd Msg)
load toShare source =
  ( { sharing = toShare
    , return = source
    , state = Input}, Cmd.none )

update : Model -> Msg -> (Model, Cmd Msg)
update shareData shareMsg =
  case shareMsg of
    DoShare email -> ({ shareData | state = Sharing }, shareCmd shareData.sharing email)
    ShareFailed error -> ({ shareData | state = Failed error }, Cmd.none)
    ShareFinished result -> ({ shareData | state = Finished result }, Cmd.none)

render: Model -> Html Msg
render model =
  let
    contents =
      case model.state of
        Input -> Html.form
          [ onSubmit (DoShare "mcclellan.mj@gmail.com")
          , classList []
          ]
          [ text "Email"
          , Html.label
            [ Html.Attributes.for "email" ]
            [ Html.input
              [ Html.Attributes.placeholder "Email"
              , Html.Attributes.name "email"
              , Html.Attributes.id "email"
              ]
              []
            ]
          , Button.button
            [ Button.primary ]
            [ Icons.share_alt, text "Share it" ]
          ]
        Sharing -> text "Sharing"
        Finished result -> text ("Finished: " ++ result.uuid)
        Failed error -> text (Errors.toText error)
  in
    div
      []
      [ Components.closeableHeader ("Share: " ++ model.sharing) <| States.generateFolderAddress model.return
      , contents
      ]

