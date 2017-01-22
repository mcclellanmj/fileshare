module ResultsExtended exposing (mapAll)

mapAll : (err -> msg) -> (a -> msg) -> Result err a -> msg
mapAll errorMapping okMapping result =
  case result of
    Ok x -> okMapping x
    Err x -> errorMapping x