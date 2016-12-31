module Service exposing (..)

import Http
import Json.Decode exposing (Decoder, string, object2, object4, bool, int, list, (:=))
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

shareFile: String -> String -> Task Http.Error (ShareResult)
shareFile path email =
  let
    url = Http.url "/share" []
  in
    Http.post parseShare url (Http.string (createShareJson path email))

fetchFiles: String -> Task Http.Error (List File)
fetchFiles path =
  let
    url =
      Http.url
        "/view"
        [("folder_path", path)]
  in
    Http.get parseFiles url

parseShare: Decoder ShareResult
parseShare =
  object2 ShareResult
    ("uuid" := string)
    ("url" := string)

parseFiles: Decoder (List File)
parseFiles =
  let
    file = object4 File
      ("short_name" := string)
      ("full_path" := string)
      ("is_folder" := bool)
      ("size" := int)
  in
    list file
