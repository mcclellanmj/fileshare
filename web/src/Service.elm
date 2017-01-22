module Service exposing (..)

import Http
import Json.Decode exposing (Decoder, string, map2, map4, bool, int, list, field)
import Json.Encode as JEncode
import Task exposing (Task)

type alias ShareResult =
  { uuid: String
  , url: String
  }

type alias File =
  { shortName: String
  , fullPath: String
  , isFolder: Bool
  , size: Int
  }

createShareJson: String -> String -> String
createShareJson path email =
  let
    shareDetails = JEncode.object
      [ ("full_path", JEncode.string path)
      , ("email", JEncode.string email)
      ]
  in
    JEncode.encode 0 shareDetails

shareFile: String -> String -> Http.Request ShareResult
shareFile path email =
  let
    url = "/share"
  in
    Http.post url (Http.stringBody "application/json" (createShareJson path email)) parseShare

fetchFiles: String -> Http.Request List File
fetchFiles path =
  let
    url = "/view?folder_path=" ++ Http.encodeUri path
  in
    Http.get parseFiles url

parseShare: Decoder ShareResult
parseShare =
  map2 ShareResult
    (field "uuid" string)
    (field "url" string)

parseFiles: Decoder (List File)
parseFiles =
  let
    file = map4 File
      (field "short_name" string)
      (field "full_path" string)
      (field "is_folder" bool)
      (field "size" int)
  in
    list file
