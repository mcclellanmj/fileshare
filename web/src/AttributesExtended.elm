module AttributesExtended exposing (voidHref)
import Html exposing (Attribute)
import Html.Attributes exposing (href)
import Html.Events

voidHref: Attribute a
voidHref = href "javascript:;"


