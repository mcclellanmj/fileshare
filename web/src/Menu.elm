module Menu exposing (render)

import Html exposing (Html, div, ul, li, text)
import Html.Attributes exposing (style, classList, href)
import Html.Events exposing (onClick, onSubmit)

renderOptions : String -> Html msg
renderOptions path =
  ul []
    [ li [] [ text "One" ]
    , li [] [ text "Two" ]
    , li [] [ text "Three" ]
    ]

render : String -> Html msg
render path =
  div []
    [ renderOptions path ]
