module UI.Components exposing (closeableHeader)

import Html exposing (Html, div, span, a, text)
import Html.Attributes exposing (href)
import Css
import FontAwesome.Web as Icons

closeableHeader: String -> String -> Html msg
closeableHeader title closeToAddress =
  div
    [ Css.withClass Css.CloseableHeader ]
    [ span [] [text title]
    , a
      [ href closeToAddress
      , Css.withClass Css.TextDanger
      ]
      [ Icons.close ]
    ]