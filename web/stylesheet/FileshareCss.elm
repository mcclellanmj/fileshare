module FileshareCss exposing (css)

import Css exposing (..)
import Css.Elements exposing (..)
import Css.Namespace exposing (namespace)
import Html.CssHelpers exposing (withNamespace)

css =
  (stylesheet << namespace (withNamespace "Cow"))
    [ header
        [ backgroundColor (rgb 90 90 90)
        , boxSizing borderBox
        , padding (px -80)
        ]
    , nav
        [ display inlineBlock
        , paddingBottom (px 12)
        ]
    ]
