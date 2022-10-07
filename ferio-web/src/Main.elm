module Main exposing (main)

import Browser exposing (Document)
import Html exposing (Html, a, div, h1, section, span, text)
import Html.Attributes exposing (class, href)
import Http
import Json.Decode exposing (Decoder, field, list, map2, map3, string)


main : Program () Model Msg
main =
    Browser.document
        { init = init
        , update = update
        , view = view
        , subscriptions = subscriptions
        }


type Model
    = Failure
    | Loading
    | Success Holidays


init : () -> ( Model, Cmd Msg )
init _ =
    ( Loading, getHolidays )


type Msg
    = GotHolidays (Result Http.Error Holidays)


type alias Holidays =
    { date : String
    , data : List Holiday
    }


type alias Holiday =
    { greeting : String
    , name : String
    , wikipedia_url : String
    }


update : Msg -> Model -> ( Model, Cmd Msg )
update msg _ =
    case msg of
        GotHolidays result ->
            case result of
                Ok holidays ->
                    ( Success holidays, Cmd.none )

                Err _ ->
                    ( Failure, Cmd.none )


view : Model -> Document Msg
view model =
    { title = "Ferio Web"
    , body =
        [ div []
            [ case model of
                Failure ->
                    text "Failure"

                Loading ->
                    text "Loading"

                Success data ->
                    viewHolidays data
            ]
        ]
    }


viewHolidays : Holidays -> Html msg
viewHolidays holidays =
    section []
        [ h1 [ class "text-left text-3xl font-bold font-mono p-5 holiday" ]
            [ span [] [ text "There are " ]
            , span [] [ text (List.length holidays.data |> String.fromInt) ]
            , span [] [ text " holidays on " ]
            , span [] [ text (String.replace "_" " " holidays.date) ]
            , span [] [ text "!" ]
            ]
        , div [ class "container mx-auto my-4 grid grid-cols-1 gap-5" ] (List.map (\holiday -> div [ class "holiday holiday-shadow" ] [ a [ href holiday.wikipedia_url ] [ text holiday.name ] ]) holidays.data)
        ]


subscriptions : Model -> Sub Msg
subscriptions _ =
    Sub.none


getHolidays : Cmd Msg
getHolidays =
    Http.get
        { url = "http://0.0.0.0:3000/"
        , expect = Http.expectJson GotHolidays holidaysDecoder
        }


holidaysDecoder : Decoder Holidays
holidaysDecoder =
    map2 Holidays
        (field "date" string)
        (field "data"
            (list
                (map3 Holiday
                    (field "greeting" string)
                    (field "name" string)
                    (field "wikipedia_url" string)
                )
            )
        )
