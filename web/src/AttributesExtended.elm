module AttributesExtended exposing (voidHref)
import Html exposing (Attribute)
import Html.Attributes exposing (href)

voidHref: Attribute a
voidHref = href "javascript:;"

