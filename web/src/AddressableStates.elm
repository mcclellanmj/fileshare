module AddressableStates exposing (AddressableState(..), decode, generateFolderAddress, generateShareAddress)

import Navigation exposing (Location)
import UrlParser exposing (Parser, parseHash, (</>), int, oneOf, s, string, custom)
import Http
import String
import Debug

type AddressableState
   = Folder String
   | Share String String

encodedUri : Parser (String -> a) a
encodedUri =
  custom "ENCODEDURI" ( Http.decodeUri >> Result.fromMaybe "Unable to parse uri" )

routeParser : Parser (AddressableState -> a) a
routeParser =
  oneOf
    [ UrlParser.map (Folder ".") UrlParser.top
    , UrlParser.map Folder (s "folder" </> encodedUri)
    , UrlParser.map (Folder ".") (s "folder")
    , UrlParser.map Share (s "share" </> encodedUri </> s "source" </> encodedUri)
    ]

fixLocationQuery : Location -> Location
fixLocationQuery location =
  let
    hash =
      String.split "?" location.hash
        |> List.head
        |> Maybe.withDefault ""

    search =
      String.split "?" location.hash
        |> List.drop 1
        |> String.join "?"
        |> String.append "?"
  in
    { location | hash = hash, search = search }

decode : Location -> Maybe AddressableState
decode location =
    parseHash routeParser (fixLocationQuery location)

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
