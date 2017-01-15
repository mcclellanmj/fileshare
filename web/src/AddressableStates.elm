module AddressableStates exposing (AddressableState(..), routeParser, decode, generateFolderAddress, generateShareAddress)

import Navigation exposing (Location)
import UrlParser exposing (Parser, parse, (</>), format, int, oneOf, s, string)
import Http
import String

type AddressableState
   = Folder String
   | Share String String

routeParser : Parser (AddressableState -> a) a
routeParser =
    oneOf
        [ format (Folder ".") (s "")
        , format (Http.uriDecode >> Folder) (s "folder" </> string)
        , format (Folder ".") (s "folder")
        , format (\share source -> Share (Http.uriDecode share) (Http.uriDecode source)) (s "share" </> string </> s "source" </> string)
        ]

decode : Location -> Result String AddressableState
decode location =
    parse identity routeParser (String.dropLeft 1 location.hash)

generatePathUrl : List (String, String) -> String
generatePathUrl parts =
  let
    encodedParts = List.map (\(x, y) -> (x, Http.uriEncode y)) parts
    queryString = List.foldl (\(x, y) sum -> sum ++ x ++ "/" ++ y ++ "/") "" encodedParts
  in
    "#" ++ queryString

generateFolderAddress : String -> String
generateFolderAddress path = generatePathUrl [("folder", path)]

generateShareAddress : String -> String -> String
generateShareAddress path source = generatePathUrl [("share", path), ("source", source)]
