module Views.Folder exposing (render, update, Model, Msg, initialModel, loadFiles)

import Service
import Files
import Html exposing (Html, div, span, text, a, ul, li)
import Html.Attributes exposing (style, classList, href)
import Html.Events exposing (onClick, onSubmit)
import FontAwesome.Web as FontAwesome
import Css
import Errors
import Http
import AddressableStates
import AttributesExtended
import ResultsExtended
import Task

type Msg
  = DirectoryFetched (List Service.File)
  | DirectoryFetchFailed Http.Error
  | ShowMenu
  | HideMenu

type MenuLink
  = Upload String
  | CreateDirectory String

type Files
  = NotLoaded
  | Loaded (List Service.File)
  | Error Http.Error

type alias Model =
  { path: Files.FilePath
  , files: Files
  , menuActive: Bool
  }

loadingModel : Files.FilePath -> Model
loadingModel filePath =
  { path = filePath
  , files = NotLoaded
  , menuActive = False
  }

initialModel : Model
initialModel = loadingModel ""

update : Model -> Msg -> (Model, Cmd Msg)
update model msg =
  case msg of
    DirectoryFetched files -> ({ model | files = Loaded files }, Cmd.none)
    DirectoryFetchFailed error -> ({ model | files = Error error }, Cmd.none)
    ShowMenu -> ({ model | menuActive = True }, Cmd.none)
    HideMenu -> ({ model | menuActive = False }, Cmd.none)

fetchCmd: String -> Cmd Msg
fetchCmd path =
  Http.send (ResultsExtended.unwrapToType DirectoryFetchFailed DirectoryFetched) (Service.fetchFiles path)

loadFiles : Files.FilePath -> (Model, Cmd Msg)
loadFiles path = ( loadingModel path, fetchCmd path )

renderHeader: Model -> Html Msg
renderHeader model =
  let
    menuAction = if model.menuActive == True then HideMenu else ShowMenu
  in
    div [ style [("padding", "5px"), ("display", "flex"), ("background-color", "black"), ("color", "white")] ]
      [ span [] [text model.path]
      , div [style [("margin-left", "auto")]]
        [ a
          [ AttributesExtended.voidHref
          , onClick menuAction
          , classList [("menu-link", True), ("menu-active", model.menuActive == True)]
          ]
          [FontAwesome.navicon]
        ]
      ]

menuLinkTo : MenuLink -> Html Msg
menuLinkTo link =
  let
    (linkText, url) = case link of
      Upload directory -> ("Upload File", AddressableStates.generateUploadAddress directory)
      CreateDirectory directory -> ("Create Directroy", AddressableStates.generateCreateAddress directory)
  in a [ href url ] [ text linkText]

renderMenu: Files.FilePath -> Html Msg
renderMenu directory =
  div [ Css.withClass Css.MenuHeader ]
    [ ul [ Css.withClass Css.MenuList ]
      [ li [] [menuLinkTo <| Upload directory]
      , li [] [menuLinkTo <| CreateDirectory directory]
      ]
    ]

renderFile: String -> Service.File -> Html Msg
renderFile currentDir file =
  let
    (url, icon, classes) =
      if file.isFolder then
        (AddressableStates.generateFolderAddress file.fullPath, FontAwesome.folder, [ ("folder-icon", True), ("type-icon", True) ])
      else
        ("/download?filename=" ++ (Http.encodeUri file.fullPath), FontAwesome.sticky_note, [ ("file-icon", True), ("type-icon", True) ])
  in
    div
      [ Css.withClass Css.FileRow ]
      [ a
        [ href url
        , Css.withClass Css.FileRowItem
        , style [ ("font-size", "1.75em") ]
        ]
        [ span
          [ style [("margin-right", ".25em")]
          , classList classes
          ]
          [ icon ]
        ]
      , div
        [ Css.withClass Css.FileHolder ]
        [ div [ Css.withClass Css.FileName ] [ a [href url] [text file.shortName] ]
        , div [ Css.withClass Css.FileDetails ] [ text "some date" ]
        ]
      , if file.isFolder == False then
          a [ href (AddressableStates.generateShareAddress file.fullPath currentDir)
            , Css.withClasses [Css.Action, Css.FileRowItem]
            ]
            [ FontAwesome.share_alt ]
        else
          span [] []
      ]

renderFiles: Files -> String -> Html Msg
renderFiles files currentDir =
  case files of
    NotLoaded -> div [] [text "Loading"]
    Loaded files -> div [] (List.map (renderFile currentDir) files)
    Error reason -> div [] [text ("Failed due to: " ++ (Errors.toText reason))]

render : Model -> Html Msg
render model =
  let
    content = if model.menuActive then renderMenu model.path else renderFiles model.files model.path
  in
    div []
      [ renderHeader model
      , content
      ]
