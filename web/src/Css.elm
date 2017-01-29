module Css exposing (CssClass(..), withClass, withClasses, withId, Id(..))

import Html exposing (Attribute)
import Html.Attributes as Attr

type CssClass
  = TypeIcon
  | FolderIcon
  | FileRow
  | FileRowItem
  | Action
  | FileHolder
  | FileName
  | FileDetails
  | MenuLink
  | MenuActive
  | TextDanger
  | MenuList
  | MenuHeader

type Id
  = Container

withId : Id -> Attribute msg
withId id =
  idString id |> Attr.id

idString : Id -> String
idString id =
  case id of
    Container -> "container"

withClasses : List CssClass -> Attribute msg
withClasses classes =
  List.map (\x -> (classString x, True)) classes |> Attr.classList

classString : CssClass -> String
classString clazz =
  case clazz of
    TypeIcon -> "type-icon"
    FolderIcon -> "folder-icon"
    FileRow -> "file-row"
    FileRowItem -> "file-row-item"
    Action -> "action"
    FileHolder -> "file-holder"
    FileName -> "file-name"
    FileDetails -> "file-details"
    MenuLink -> "menu-link"
    MenuActive -> "menu-active"
    TextDanger -> "text-danger"
    MenuHeader -> "menu-header"
    MenuList -> "menu-list"

withClass : CssClass -> Attribute msg
withClass clazz = Attr.class (classString clazz)
