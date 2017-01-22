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
import FontAwesome.Web as FontAwesome
import Css exposing (withClass, withClasses, CssClass(..), withId, Id(..))
import Menu
import Views.Share
import Views.Folder

-- Model
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

type ViewModel
  = ShareModel Views.Share.Model
  | FolderModel Views.Folder.Model
  | ErrorModel String

initialModel : Model
initialModel =
  { viewModel = FolderModel Views.Folder.initialModel }

init : Result String AddressableState -> ( Model, Cmd Msg )
init result = urlUpdate result initialModel

type Msg
  = FolderMsg Views.Folder.Msg
  | ShareMsg Views.Share.Msg

-- Service
main : Program Never
main =
  Navigation.program (Navigation.makeParser AddressableStates.decode)
    { init = init
    , subscriptions = \_ -> Sub.none
    , update = update
    , urlUpdate = urlUpdate
    , view = view
    }

update: Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    FolderMsg m ->
      case model.viewModel of
        FolderModel folderModel -> fromFolder model (Views.Folder.update folderModel m)
        -- If we got a Folder Msg but we are not in the Folder View, disregard its late and the user has moved on
        _ -> (model, Cmd.none)

    ShareMsg m -> (model, Cmd.none)

fromFolder : Model -> (Views.Folder.Model, Cmd Views.Folder.Msg) -> (Model, Cmd Msg)
fromFolder curModel (viewModel, viewCmd) =
  let
    newModel = { curModel | viewModel = FolderModel viewModel }
  in
    (newModel, Cmd.map FolderMsg viewCmd)

fromShare : Model -> (Views.Share.Model, Cmd Views.Share.Msg) -> (Model, Cmd Msg)
fromShare curModel (viewModel, viewCmd) =
  let
    newModel = { curModel | viewModel = ShareModel viewModel }
  in
    (newModel, Cmd.map ShareMsg viewCmd)

urlUpdate : Result String AddressableState -> Model -> ( Model, Cmd Msg )
urlUpdate result model =
    case result of
      Err x -> ({ model | viewModel = ErrorModel x}, Cmd.none)
      Ok (AddressableStates.Folder s) -> fromFolder model (Views.Folder.loadFiles s)
      Ok (AddressableStates.Share toShare sourcePath) -> fromShare model (Views.Share.load toShare sourcePath)

-- View
view : Model -> Html Msg
view model =
  let
    contents = case model.viewModel of
      FolderModel viewModel -> App.map FolderMsg (Views.Folder.render viewModel)
      ShareModel viewModel -> App.map ShareMsg (Views.Share.render viewModel)
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
