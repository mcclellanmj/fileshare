import Html exposing (Html, Attribute, div, h1, input, text, ul, li, a, span, img, textarea)
import Task
import Navigation
import AddressableStates exposing (AddressableState(..))
import Css exposing (withClass, withClasses, CssClass(..), withId, Id(..))
import Views.Share
import Views.Folder
import Views.Upload
import Views.CreateDir

-- Model
type alias Model = { componentModel: ComponentModel }

type ComponentModel
  = ShareModel Views.Share.Model
  | FolderModel Views.Folder.Model
  | UploadModel Views.Upload.Model
  | CreateDirModel Views.CreateDir.Model
  | ErrorModel String

initialModel : Model
initialModel =
  { componentModel = FolderModel Views.Folder.initialModel }

init : Navigation.Location -> ( Model, Cmd Msg )
init location = ( initialModel, Task.perform UrlChange (Task.succeed location) )

type Msg
  = FolderMsg Views.Folder.Msg
  | ShareMsg Views.Share.Msg
  | UploadMsg Views.Upload.Msg
  | CreateDirMsg Views.CreateDir.Msg
  | UrlChange Navigation.Location

-- Service
main : Program Never Model Msg
main =
  Navigation.program UrlChange
    { init = init
    , update = update
    , view = view
    , subscriptions = subscriptions
    }

subscriptions : Model -> Sub Msg
subscriptions currentModel =
  case currentModel.componentModel of
    UploadModel uploadModel -> Sub.map UploadMsg (Views.Upload.subscriptions uploadModel)
    _ -> Sub.none

urlUpdate : Model -> Navigation.Location -> (Model, Cmd Msg)
urlUpdate model newLocation =
  case AddressableStates.decode newLocation of
    Nothing -> ({ model | componentModel = ErrorModel ("Unknown url [" ++ String.dropLeft 1 newLocation.hash ++ "]") }, Cmd.none)
    Just (AddressableStates.Folder path) -> mapFolderUpdate model (Views.Folder.loadFiles path)
    Just (AddressableStates.Share toShare sourcePath) -> mapShareUpdate model (Views.Share.load toShare sourcePath)
    Just (AddressableStates.Upload toUploadTo) -> mapUploadUpdate model (Views.Upload.load toUploadTo)
    Just (AddressableStates.CreateDir toCreateIn) -> ( { model | componentModel = ErrorModel "Create Not Implemented" }, Cmd.none)

update: Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    FolderMsg componentMsg ->
      case model.componentModel of
        FolderModel folderModel -> mapFolderUpdate model (Views.Folder.update folderModel componentMsg)
        _ -> (model, Cmd.none)

    ShareMsg componentMsg ->
      case model.componentModel of
        ShareModel shareModel -> mapShareUpdate model (Views.Share.update shareModel componentMsg)
        _ -> (model, Cmd.none)

    UploadMsg componentMsg ->
      case model.componentModel of
        UploadModel uploadModel -> mapUploadUpdate model (Views.Upload.update uploadModel componentMsg)
        _ -> (model, Cmd.none)

    CreateDirMsg componentMsg ->
      case model.componentModel of
        CreateDirModel uploadModel -> mapCreateDirUpdate model (Views.CreateDir.update uploadModel componentMsg)
        _ -> (model, Cmd.none)

    UrlChange location -> urlUpdate model location

mapComponentUpdate : (a -> ComponentModel) -> (b -> Msg) -> Model -> (a, Cmd b) -> (Model, Cmd Msg)
mapComponentUpdate viewFn msgFn model (componentModel, viewCmd) =
  ({ model | componentModel = viewFn componentModel }, Cmd.map msgFn viewCmd )

mapFolderUpdate : Model -> ( Views.Folder.Model, Cmd Views.Folder.Msg ) -> ( Model, Cmd Msg )
mapFolderUpdate = mapComponentUpdate FolderModel FolderMsg

mapShareUpdate : Model -> ( Views.Share.Model, Cmd Views.Share.Msg ) -> ( Model, Cmd Msg )
mapShareUpdate = mapComponentUpdate ShareModel ShareMsg

mapUploadUpdate : Model -> ( Views.Upload.Model, Cmd Views.Upload.Msg ) -> ( Model, Cmd Msg )
mapUploadUpdate = mapComponentUpdate UploadModel UploadMsg

mapCreateDirUpdate : Model -> ( Views.CreateDir.Model, Cmd Views.CreateDir.Msg ) -> ( Model, Cmd Msg )
mapCreateDirUpdate = mapComponentUpdate CreateDirModel CreateDirMsg

-- View
view : Model -> Html Msg
view model =
  let
    contents = case model.componentModel of
      FolderModel componentModel -> Html.map FolderMsg (Views.Folder.render componentModel)
      ShareModel componentModel -> Html.map ShareMsg (Views.Share.render componentModel)
      UploadModel componentModel -> Html.map UploadMsg (Views.Upload.render componentModel)
      CreateDirModel componentModel -> Html.map CreateDirMsg (Views.CreateDir.render componentModel)
      ErrorModel reason -> div [] [text reason]
  in
    div [ withId Container ] [ contents ]
