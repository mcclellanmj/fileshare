module Errors exposing (toText)

import Http

toText: Http.Error -> String
toText err =
  case err of
    Http.Timeout -> "Timeout"
    Http.NetworkError -> "Network Error"
    Http.BadUrl _ -> "Bad Url"
    Http.BadStatus _ -> "Bad Status"
    Http.BadPayload _ _ -> "Bad Payload"
