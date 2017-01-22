module Errors exposing (toText)

import Http

toText: Http.Error -> String
toText err =
  case err of
    Http.Timeout -> "Timeout"
    Http.NetworkError -> "Network Error"
    Http.UnexpectedPayload s -> "Unexpected Payload: " ++ s
    Http.BadResponse code r -> (toString code) ++ " " ++ r
