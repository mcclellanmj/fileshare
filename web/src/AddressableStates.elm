module AddressableStates exposing (AddressableState(..), routeParser, decode, generateFolderAddress)

import Navigation exposing (Location)
import UrlParser exposing (Parser, parse, (</>), format, int, oneOf, s, string)
import Http
import String

type AddressableState
   = Folder String

routeParser : Parser (AddressableState -> a) a
routeParser =
    oneOf
        [ format (Folder ".") (s "")
        , format (Http.uriDecode >> Folder) (s "folder" </> string)
        , format (Folder ".") (s "folder")
        ]

decode : Location -> Result String AddressableState
decode location =
    parse identity routeParser (String.dropLeft 1 location.hash)

generatePathUrl : String -> String -> String
generatePathUrl section path = "#" ++ section ++ "/" ++ (Http.uriEncode path)

generateFolderAddress : String -> String
generateFolderAddress path = generatePathUrl "folder" path
