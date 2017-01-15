module SharePrompt exposing (render)

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

render: (String -> String -> a) -> String -> String -> Html a
render shareFn path source =
  div
  []
  [ shareHeader source ("Share: " ++ path)
  , Html.form
    [ onSubmit (shareFn path "mcclellan.mj@gmail.com")
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

