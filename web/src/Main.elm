import Html.App as App
import Html.Attributes exposing (href, style, src, classList)
import Html.Events exposing (onClick, onSubmit)
import Html exposing (Html, Attribute, div, h1, input, text, ul, li, a, span, img, textarea)
import Window
import Task
import Http exposing (Error)
import Service exposing (File, ShareResult, fetchFiles)
import Json.Decode exposing (Decoder, object4, string, int, bool, list, (:=))
import Navigation
import AddressableStates exposing (AddressableState(..))
import AttributesExtended
import SharePrompt
import FontAwesome.Web as FontAwesome
import Css exposing (withClass, withClasses, CssClass(..), withId, Id(..))
import Menu

-- Model
type Files
  = NotLoaded
  | Loaded (List File)
  | Error Http.Error

type Prompt
  = None
  | Menu
  | Share String String
  | Shared ShareResult
  | Sharing String String
  | FailedShare Http.Error

type RouteModel
  = Loading
  | BadRoute String
  | Folder

type alias FilePath = String

type alias Model = { viewModel: ViewModel }

type alias ShareData =
  { sharing: FilePath
  , return: FilePath
  }

type alias MenuData = { active: Bool }

type alias FolderData =
  { path: FilePath
  , files: Files
  , menu: MenuData
  }

type ViewModel
  = ShareModel ShareData
  | FolderModel FolderData
  | ErrorModel String

initialFolderData : FolderData
initialFolderData =
  { path = ""
  , files = NotLoaded
  , menu = { active = False }
  }

initialModel : Model
initialModel =
  { viewModel = FolderModel initialFolderData }

init : Result String AddressableState -> ( Model, Cmd Msg )
init result = urlUpdate result initialModel

type Msg
  = DirectoryFetched (List File)
  | DirectoryFetchFailed Http.Error
  | WindowResize (Int, Int)
  | WindowSizeFailed
  | ShowMenu
  | HideMenu
  | ShowSharePrompt String String
  | ShareMsg String String
  | ShareFailed Http.Error
  | ShareFinished ShareResult

-- Service
main : Program Never
main =
  Navigation.program (Navigation.makeParser AddressableStates.decode)
    { init = init
    , subscriptions = \_ -> Window.resizes windowChange
    , update = update
    , urlUpdate = urlUpdate
    , view = view
    }

getCurrentWindowSize: Cmd Msg
getCurrentWindowSize = Task.perform (\_ -> WindowSizeFailed) windowChange Window.size

fetchCmd: String -> Cmd Msg
fetchCmd path =
    Task.perform DirectoryFetchFailed DirectoryFetched (Service.fetchFiles path)

shareCmd: String -> String -> Cmd Msg
shareCmd path email = Task.perform ShareFailed ShareFinished (Service.shareFile path email)

windowChange: Window.Size -> Msg
windowChange dim = WindowResize (dim.width, dim.height)

update: Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    DirectoryFetchFailed r -> ({ model | files=Error r}, Cmd.none)
    DirectoryFetched x -> ({ model | files=Loaded x}, Cmd.none)
    WindowResize size -> ({ model | windowSize=Just size}, Cmd.none)
    WindowSizeFailed -> ({ model | windowSize=Nothing}, Cmd.none)
    ShareMsg path email -> ({ model | prompt=Sharing path email}, shareCmd path email)
    ShowSharePrompt path source -> ({ model | prompt=Share path source }, Cmd.none)
    ShareFailed r -> ({ model | prompt=FailedShare r}, Cmd.none)
    ShareFinished result -> ({ model | prompt=Shared result}, Cmd.none)
    ShowMenu -> ({model | prompt=Menu}, Cmd.none)
    HideMenu -> ({ model | prompt=None }, Cmd.none)

loadingFolderModel : FilePath -> ViewModel
loadingFolderModel filepath =
  FolderModel
    { path = filepath
    , files = NotLoaded
    , menu = { active = False }
    }

shareModel : FilePath -> FilePath -> ViewModel
shareModel toShare toReturnTo =
  ShareModel
    { sharing = toShare
    , return = toReturnTo
    }

urlUpdate : Result String AddressableState -> Model -> ( Model, Cmd Msg )
urlUpdate result model =
    case result of
      Err x -> ({ model | viewModel = ErrorModel x}, Cmd.none)
      Ok (AddressableStates.Folder s) -> ({ model | viewModel = loadingFolderModel s}, fetchCmd s)
      Ok (AddressableStates.Share toShare sourcePath) -> ({ model | viewModel = shareModel toShare sourcePath}, Cmd.none)

-- View
renderCoords: (Int, Int) -> String
renderCoords (x, y) =
  "(" ++ toString x ++ "," ++ toString y ++ ")"

renderFile: String -> File -> Html Msg
renderFile currentDir file =
  let
    (url, icon, classes) =
      if file.isFolder then
        (AddressableStates.generateFolderAddress file.fullPath, FontAwesome.folder, [ ("folder-icon", True), ("type-icon", True) ])
      else
        (Http.url "/download" [("filename", file.fullPath)], FontAwesome.sticky_note, [ ("file-icon", True), ("type-icon", True) ])
  in
    div
      [ withClass FileRow ]
      [ a
        [ href url
        , withClass FileRowItem
        , style [ ("font-size", "1.75em") ]
        ]
        [ span
          [ style [("margin-right", ".25em")]
          , classList classes
          ]
          [ icon ]
        ]
      , div
        [ withClass FileHolder ]
        [ div [ withClass FileName ] [ a [href url] [text file.shortName] ]
        , div [ withClass FileDetails ] [ text "some date" ]
        ]
      , if file.isFolder == False then
          a [ href (AddressableStates.generateShareAddress file.fullPath currentDir)
            , withClasses [Action, FileRowItem]
            ]
            [ FontAwesome.share_alt ]
        else
          span [] []
      ]

toErrorText: Http.Error -> String
toErrorText err =
  case err of
    Http.Timeout -> "Timeout"
    Http.NetworkError -> "Network Error"
    Http.UnexpectedPayload s -> "Unexpected Payload: " ++ s
    Http.BadResponse code r -> (toString code) ++ " " ++ r

renderFiles: Files -> String -> Html Msg
renderFiles files currentDir =
  case files of
    NotLoaded -> div [] [text "Loading"]
    Loaded files -> div [] (List.map (renderFile currentDir) files)
    Error reason -> div [] [text ("Failed due to: " ++ (toErrorText reason))]

renderFileHeader: FolderData -> Html Msg
renderFileHeader model =
  let
    menuAction = if model.menu.active == True then HideMenu else ShowMenu
  in
    div [ style [("padding", "5px"), ("display", "flex"), ("background-color", "black"), ("color", "white")] ]
      [ span [] [text model.path]
      , div [style [("margin-left", "auto")]]
        [ a
          [ AttributesExtended.voidHref
          , onClick menuAction
          , classList [("menu-link", True), ("menu-active", model.menu.active == True)]
          ]
          [FontAwesome.navicon]
        ]
      ]

view : Model -> Html Msg
view model =
  let
    contents = case model.viewModel of
      FolderModel folderData ->
        div []
          [ renderFileHeader folderData
          , renderFiles folderData.files folderData.path
          ]

      ShareModel shareData -> div [] [text "ToDo"]
          -- Share file source -> SharePrompt.render ShareMsg file source
          -- Shared result ->
            -- div
              -- []
              -- [ div [] [text "Email has been sent"] ]
          -- Sharing file email -> div [] [text "Sharing it"]
          -- FailedShare reason -> div [] [text "Failed it"]
          -- Menu -> Menu.render model.path

      -- BadRoute reason -> div [] [text reason]
      _ -> div [] []
  in
    div [ withId Container ] [ contents ]
