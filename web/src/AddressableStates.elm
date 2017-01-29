module AddressableStates exposing
  ( AddressableState(..)
  , decode
  , generateFolderAddress
  , generateShareAddress
  , generateCreateAddress
  , generateUploadAddress)

import Navigation exposing (Location)
import UrlParser exposing (Parser, parseHash, (</>), int, oneOf, s, string, custom)
import Http
import String
import Debug

type AddressableState
   = Folder String
   | Share String String
   | Upload String
   | CreateDir String

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
    , UrlParser.map Upload (s "upload" </> encodedUri)
    , UrlParser.map CreateDir (s "create" </> encodedUri)
    ]

decode : Location -> Maybe AddressableState
decode location =
    parseHash routeParser location

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

generateUploadAddress : String -> String
generateUploadAddress path = generatePathUrl [("upload", path)]

generateCreateAddress : String -> String
generateCreateAddress path = generatePathUrl [("create", path)]
