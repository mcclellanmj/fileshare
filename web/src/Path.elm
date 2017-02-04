module Path exposing (parsePath, getSepChar, takeFilename, takeParent, appendPath, toString, equals)
{-| Doing work on paths
@docs parsePath, getSepChar, takeFilename, takeParent, appendPath, toString, equals
-}

type FileSystem
  = Unix
  | Windows

type alias FilePath = List String

{-| Gets the separator to be used
-}
getSepChar : FileSystem -> String
getSepChar sep =
  case sep of
    Unix -> "/"
    Windows -> "\\"

{-| Parses a path
    parsePath "test/test/" = ["test", "test", ""]
-}
parsePath : FileSystem -> String -> FilePath
parsePath sep unparsed = String.split (getSepChar sep) unparsed

{-| Gets the parent
-}
takeParent : FilePath -> Maybe FilePath
takeParent filepath =
  List.reverse filepath
    |> List.tail
    |> Maybe.map List.reverse

{-| Returns the filename
-}
takeFilename : FilePath -> Maybe String
takeFilename filepath = List.head (List.reverse filepath)

{-| Append a file -}
appendPath : FilePath -> FilePath -> FilePath
appendPath source toAdd =
  case source of
    _ :: "" :: [] -> List.append (Maybe.withDefault [] (takeParent source)) toAdd
    x -> List.append x toAdd

{-| As String -}
toString : FileSystem -> FilePath -> String
toString sep path =
  String.join (getSepChar sep) path

{-| Check if two filepaths are equivalent -}
equals : FileSystem -> FilePath -> FilePath -> Bool
equals sys path1 path2 =
  case sys of
    Unix -> path1 == path2
    Windows -> List.map String.toLower path1 == List.map String.toLower path2