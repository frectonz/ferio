module Main exposing (main)

import Browser exposing (Document)
import Html exposing (Html, a, div, h1, li, option, select, text, ul)
import Html.Attributes exposing (href, selected, style, value)
import Html.Events exposing (onInput)
import Http
import Json.Decode as Decode exposing (Decoder, field, list, map2, map3, string)


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
    ( Loading, getHolidays Nothing )


type Msg
    = GotHolidays (Result Http.Error Holidays)
    | GotMonth String
    | GotDay String


type alias Holidays =
    { date : Date
    , data : List Holiday
    }


type alias Date =
    { month : String, day : Int }


type alias Holiday =
    { greeting : String
    , name : String
    , wikipedia_url : String
    }


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case ( msg, model ) of
        ( GotHolidays result, _ ) ->
            case result of
                Ok holidays ->
                    ( Success holidays, Cmd.none )

                Err _ ->
                    ( Failure, Cmd.none )

        ( GotMonth month, Success m ) ->
            ( Loading, getHolidays (Just { month = month, day = m.date.day }) )

        ( GotDay day, Success m ) ->
            ( Loading
            , getHolidays
                (Just
                    { month = m.date.month
                    , day = String.toInt day |> Maybe.withDefault 0
                    }
                )
            )

        _ ->
            ( model, Cmd.none )


view : Model -> Document Msg
view model =
    let
        holidays =
            case model of
                Failure ->
                    viewError

                Loading ->
                    viewLoading

                Success data ->
                    viewHolidays data

        pageTitle =
            case model of
                Failure ->
                    "Oops!"

                Loading ->
                    "Loading Holidays"

                Success data ->
                    "Holidays of " ++ data.date.month ++ " " ++ String.fromInt data.date.day
    in
    { title = pageTitle
    , body = [ holidays ]
    }


viewHolidays : Holidays -> Html Msg
viewHolidays holidays =
    div []
        [ h1 []
            [ text
                ("There are "
                    ++ (List.length holidays.data |> String.fromInt)
                    ++ " holidays on "
                )
            , monthInput holidays.date.month
            , dayInput holidays.date.day
            ]
        , ul [] (holidays.data |> List.map viewHoliday)
        ]


viewHoliday : Holiday -> Html msg
viewHoliday holiday =
    li []
        [ a [ href holiday.wikipedia_url ] [ text holiday.name ]
        ]


viewError : Html msg
viewError =
    h1 [] [ text "Sorry, something went wrong!" ]


viewLoading : Html msg
viewLoading =
    h1 [] [ text "Loading Holidays..." ]


monthInput : String -> Html Msg
monthInput month =
    select [ style "font-size" "0.7em", onInput GotMonth ]
        ([ "January", "Feburary", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December" ]
            |> List.map
                (\x ->
                    option [ value x, selected (x == month) ] [ text x ]
                )
        )


dayInput : Int -> Html Msg
dayInput day =
    select [ style "font-size" "0.7em", onInput GotDay ]
        (List.repeat 31 0
            |> List.indexedMap (\i _ -> String.fromInt i)
            |> List.map
                (\i ->
                    option [ value i, selected (i == String.fromInt day) ]
                        [ text i ]
                )
        )


subscriptions : Model -> Sub Msg
subscriptions _ =
    Sub.none


getHolidays : Maybe Date -> Cmd Msg
getHolidays d =
    Http.get
        { url =
            "http://ferio-api.frectonz.io"
                ++ (case d of
                        Just { month, day } ->
                            "/?date=" ++ month ++ "_" ++ String.fromInt day

                        Nothing ->
                            "/"
                   )
        , expect = Http.expectJson GotHolidays holidaysDecoder
        }


holidaysDecoder : Decoder Holidays
holidaysDecoder =
    map2 Holidays
        (field "date" dateDecoder)
        (field "data"
            (list
                (map3 Holiday
                    (field "greeting" string)
                    (field "name" string)
                    (field "wikipedia_url" string)
                )
            )
        )


dateDecoder : Decoder Date
dateDecoder =
    string
        |> Decode.andThen
            (\str ->
                case String.split "_" str of
                    [ month, day ] ->
                        Decode.succeed ( month, day )

                    _ ->
                        Decode.fail "Invalid date"
            )
        |> Decode.andThen
            (\( month, day ) ->
                case String.toInt day of
                    Just dayInt ->
                        Decode.succeed (Date month dayInt)

                    Nothing ->
                        Decode.fail "date is not an integer"
            )
