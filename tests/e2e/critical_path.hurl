# registration
POST {{host}}/{{version}}/registration
{"email": "{{a_string}}@{{a_string}}.iv","password": "AbcD1x!#"}

HTTP 200

# login
POST {{host}}/{{version}}/login
{"email": "{{a_string}}@{{a_string}}.iv", "password": "AbcD1x!#"}

HTTP 200

[Captures]
token: regex "\"([^\"]*)\""

# check store in db
POST {{sql_query_path}}
 "select id from accounts where email ='{{a_string}}@{{a_string}}.iv';"

HTTP 200

[Captures]
accountID: regex "(\\d+)"

# add activity
POST {{host}}/{{version}}/activity
Authorization: {{token}}

{
    "title": "{{title}}",
    "content": "{{content}}",
    "time": {{time}}
}

HTTP 200

# check DB store this
POST {{sql_query_path}}
"select title, id from activities where account_id={{accountID}};"

HTTP 200

[Asserts]
body contains "{{title}}"

