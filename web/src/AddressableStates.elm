module AddressableStates exposing (AddressableState(..), routeParser, decode, generateFolderAddress, generateShareAddress)

import Navigation exposing (Location)
import UrlParser exposing (Parser, parseHash, (</>), int, oneOf, s, string)
import Http
import String

type AddressableState
   = Folder String
   | Share String String

routeParser : Parser (AddressableState -> a) a
routeParser =
    oneOf
        [ UrlParser.map (Folder ".") (s "")
        , UrlParser.map (Http.decodeUri >> Folder) (s "folder" </> string)
        , UrlParser.map (Folder ".") (s "folder")
        , UrlParser.map (\share source -> Share (Http.decodeUri share) (Http.decodeUri source)) (s "share" </> string </> s "source" </> string)
        ]

decode : Location -> Result String AddressableState
decode location =
    parseHash identity routeParser (String.dropLeft 1 location.hash)

generatePathUrl : List (String, String) -> String
generatePathUrl parts =
  let
    encodedParts = List.map (\(x, y) -> (x, Http.encodeUri y)) parts
    queryString = List.foldl (\(x, y) sum -> sum ++ x ++ "/" ++ y ++ "/") "" encodedParts
  in
    "#" ++ queryString

generateFolderAddress : String -> String
generateFolderAddress path = generatePathUrl [("folder", path)]

generateShareAddress : String -> String -> String
generateShareAddress path source = generatePathUrl [("share", path), ("source", source)]
