module ResultsExtended exposing (unwrapToType)

unwrapToType : (err -> msg) -> (a -> msg) -> Result err a -> msg
unwrapToType errorMapping okMapping result =
  case result of
    Ok x -> okMapping x
    Err x -> errorMapping x