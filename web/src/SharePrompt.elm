module SharePrompt exposing (render)

import Html exposing (Html, div, span, text, a)
import Html.Attributes exposing (style, classList)
import Html.Events exposing (onClick, onSubmit)
import Service exposing (File)
import AttributesExtended as AttrExt
import FontAwesome.Web as Icons
import Pure
import UI.Colors as Colors

shareHeader: a -> String -> Html a
shareHeader closeMsg title =
  div [style [("display", "flex"), ("background-color", "black"), ("padding", "5px"), ("color", "white")]]
    [ span [] [text title]
    , a
      [ AttrExt.voidHref
      , onClick closeMsg
      , style
        [ ("margin-left", "auto")
        , ("color", Colors.dangerText)
        ]
      , classList [("text-danger", True)]
      ]
      [ Icons.close ]
    ]

render: a -> (File -> String -> a) -> File -> Html a
render closeMsg shareFn file =
  div
  [ style [("max-width", "400px")] ]
  [ shareHeader closeMsg ("Share: " ++ file.shortName)
  , Html.form
    [ onSubmit (shareFn file "mcclellan.mj@gmail.com")
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

