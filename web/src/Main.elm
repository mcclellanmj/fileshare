import Html.Attributes exposing (href, style, src, classList)
import Html.Events exposing (onClick, onSubmit)
import Html exposing (Html, Attribute, div, h1, input, text, ul, li, a, span, img, textarea)
import Task
import Http exposing (Error)
import Service exposing (File, ShareResult, fetchFiles)
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

init : Navigation.Location -> ( Model, Cmd Msg )
init location = ( initialModel, Task.perform UrlChange (Task.succeed location) )

type Msg
  = FolderMsg Views.Folder.Msg
  | ShareMsg Views.Share.Msg
  | UrlChange Navigation.Location

-- Service
main =
  Navigation.program UrlChange
    { init = init
    , update = update
    , view = view
    , subscriptions = \_ -> Sub.none
    }

urlUpdate: Model -> Navigation.Location -> (Model, Cmd Msg)
urlUpdate model newLocation =
  case AddressableStates.decode newLocation of
    Nothing -> ({ model | viewModel = ErrorModel ("Unknown url [" ++ String.dropLeft 1 newLocation.hash ++ "]") }, Cmd.none)
    Just (AddressableStates.Folder path) -> fromFolder model (Views.Folder.loadFiles path)
    Just (AddressableStates.Share toShare sourcePath) -> fromShare model (Views.Share.load toShare sourcePath)

update: Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    FolderMsg m ->
      case model.viewModel of
        FolderModel folderModel -> fromFolder model (Views.Folder.update folderModel m)
        -- If we got a Folder Msg but we are not in the Folder View, disregard its late and the user has moved on
        _ -> (model, Cmd.none)

    ShareMsg m ->
      case model.viewModel of
        ShareModel shareModel -> fromShare model (Views.Share.update shareModel m)
        -- If we got a Folder Msg but we are not in the Folder View, disregard its late and the user has moved on
        _ -> (model, Cmd.none)

    UrlChange location -> urlUpdate model location

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

-- View
view : Model -> Html Msg
view model =
  let
    contents = case model.viewModel of
      FolderModel viewModel -> Html.map FolderMsg (Views.Folder.render viewModel)
      ShareModel viewModel -> Html.map ShareMsg (Views.Share.render viewModel)
          -- Share file source -> SharePrompt.render ShareMsg file source
          -- Shared result ->
            -- div
              -- []
              -- [ div [] [text "Email has been sent"] ]
          -- Sharing file email -> div [] [text "Sharing it"]
          -- FailedShare reason -> div [] [text "Failed it"]
          -- Menu -> Menu.render model.path

      ErrorModel reason -> div [] [text reason]
  in
    div [ withId Container ] [ contents ]
