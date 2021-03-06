module Views.Folder exposing (render, update, Model, Msg, initialModel, loadFiles, subscriptions)

import Service
import Files
import Html exposing (Html, div, span, text, a, ul, li)
import Html.Attributes exposing (style, classList, href)
import Html.Events exposing (onClick)
import FontAwesome.Web as FontAwesome
import Css
import Errors
import Http
import AddressableStates
import AttributesExtended
import Result.Extra
import Modals.CreateDir

import Bootstrap.Navbar as Navbar

type Msg
  = DirectoryFetched (List Service.File)
  | DirectoryFetchFailed Http.Error
  | NavbarMsg Navbar.State
  | CreateDirMsg Modals.CreateDir.Msg

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
  , navbarState: Navbar.State
  , modalState: Modals.CreateDir.Model
  }

loadingModel : Files.FilePath -> (Model, Cmd Msg)
loadingModel filePath =
  let
    (navbarState, navbarCmd) = Navbar.initialState NavbarMsg
  in
    ({ path = filePath
    , files = NotLoaded
    , menuActive = False
    , navbarState = navbarState
    , modalState = Modals.CreateDir.initialState
    }, navbarCmd)

initialModel : (Model, Cmd Msg)
initialModel = loadingModel ""

update : Model -> Msg -> (Model, Cmd Msg)
update model msg =
  case msg of
    DirectoryFetched files -> ({ model | files = Loaded files }, Cmd.none)
    DirectoryFetchFailed error -> ({ model | files = Error error }, Cmd.none)
    NavbarMsg state -> ({ model | navbarState = state }, Cmd.none)
    CreateDirMsg createDirMsg ->
      let
        (newModel, cmd) = Modals.CreateDir.update model.modalState createDirMsg
      in
        ({ model | modalState = newModel }, Cmd.map CreateDirMsg cmd)

fetchCmd: String -> Cmd Msg
fetchCmd path =
  Http.send (Result.Extra.unpack DirectoryFetchFailed DirectoryFetched) (Service.fetchFiles path)

loadFiles : Files.FilePath -> (Model, Cmd Msg)
loadFiles path =
  let
    (initialModel, navbarCmd) = loadingModel path
  in
    ( initialModel, Cmd.batch [fetchCmd path, navbarCmd] )

renderHeader: Model -> Html Msg
renderHeader model =
  Navbar.config NavbarMsg
    |> Navbar.withAnimation
    |> Navbar.brand [ AttributesExtended.voidHref ] [ text model.path ]
    |> Navbar.items
      [ Navbar.itemLink [ href (AddressableStates.generateUploadAddress model.path) ] [ text "Upload File" ]
      , Navbar.itemLink [ AttributesExtended.voidHref, onClick (CreateDirMsg (Modals.CreateDir.ShowModal model.path) )] [ text "CreateDirectory" ]
      ]
    |> Navbar.view model.navbarState

menuLinkTo : MenuLink -> Html Msg
menuLinkTo link =
  let
    (linkText, url) = case link of
      Upload directory -> ("Upload File", AddressableStates.generateUploadAddress directory)
      CreateDirectory directory -> ("Create Directroy", AddressableStates.generateCreateAddress directory)
  in a [ href url ] [ text linkText]

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
    content = renderFiles model.files model.path
  in
    div []
      [ renderHeader model
      , Html.map CreateDirMsg <| Modals.CreateDir.view model.modalState
      , content
      ]

subscriptions : Model -> Sub Msg
subscriptions model =
  Navbar.subscriptions model.navbarState NavbarMsg
