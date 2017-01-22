module Views.Share exposing (render, Model, Msg, update, load)

import Html exposing (Html, div, span, text, a)
import Html.Attributes exposing (style, classList, href)
import Html.Events exposing (onClick, onSubmit)
import Service exposing (File)
import AttributesExtended as AttrExt
import FontAwesome.Web as Icons
import Pure
import UI.Colors as Colors
import Css as ShareCss
import AddressableStates as States
import Files exposing (FilePath)
import Service exposing (ShareResult)
import Http
import Task

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
shareCmd path email = Task.perform ShareFailed ShareFinished (Service.shareFile path email)

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

shareHeader: String -> String -> Html a
shareHeader source title =
  div [style [("display", "flex"), ("background-color", "black"), ("padding", "5px"), ("color", "white")]]
    [ span [] [text title]
    , a
      [ href (States.generateFolderAddress source)
      , style
        [ ("margin-left", "auto")
        , ("color", Colors.dangerText)
        ]
      , ShareCss.withClass ShareCss.TextDanger
      ]
      [ Icons.close ]
    ]

render: Model -> Html Msg
render model =
  div
  []
  [ shareHeader model.return ("Share: " ++ model.sharing)
  , Html.form
    [ onSubmit (DoShare "mcclellan.mj@gmail.com")
    , classList [ (Pure.formStacked, True)] ]
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
    , Html.button
      [ classList [ (Pure.button, True), (Pure.buttonPrimary, True) ]
      ]
      [ Icons.share_alt, text "Share it" ]
    ]
  ]

