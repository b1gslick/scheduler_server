# registration
POST {{host}}/{{version}}/registration
{"email": "{{email_1}}","password": "AbcD1x!#"}

HTTP 201

# login
POST {{host}}/{{version}}/login
{"email": "{{email_1}}", "password": "AbcD1x!#"}

HTTP 200

[Captures]
token: jsonpath "$.token"

# check store in db
POST {{sql_query_path}}
 "select id from accounts where email ='{{email_1}}';"

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

HTTP 201

[Asserts]
jsonpath "$.title" == {{title}}
jsonpath "$.content" == {{content}}
jsonpath "$.time" == {{time}}

# check DB store this
POST {{sql_query_path}}
"select title, id from activities where account_id={{accountID}};"

HTTP 200

[Captures]
id: jsonpath "$[0].id"

[Asserts]
body contains "{{title}}"
body contains "{{id}}"

# check activity added
GET {{host}}/{{version}}/activity?limit={{limit}}&offset={{offset}}
Authorization: {{token}}

HTTP 200

[Asserts]
jsonpath "$..title" contains {{title}}
jsonpath "$..content" contains {{content}}

# update activity first time
PUT {{host}}/{{version}}/activity/{{id}}
Authorization: {{token}}

{
    "title": "{{new_title}}",
    "content": "{{new_content}}",
    "time": {{new_time}}
}

HTTP 200

[Asserts]
jsonpath "$.id" toString == {{id}}

# check activity updated

GET {{host}}/{{version}}/activity?limit={{limit}}&offset={{offset}}
Authorization: {{token}}

HTTP 200

[Asserts]
jsonpath "$..title" contains {{new_title}}
jsonpath "$..content" contains {{new_content}}

# add time spent

POST {{host}}/{{version}}/time_spent
Authorization: {{token}}

{
  "time": {{time}},
  "activity_id": {{id}}
}

HTTP 200

[Asserts]
body contains "{{time}}"

# time spent
POST {{sql_query_path}}

"select id, activity_id from time_spent where activity_id={{id}};"


HTTP 200

[Captures]
timeSpentID: jsonpath "$[0].id"

[Asserts]
jsonpath "$[0].activity_id" == {{id}}

# check time spent added
GET {{host}}/{{version}}/time_spent/{{timeSpentID}}
Authorization: {{token}}

HTTP 200

[Asserts]
jsonpath "$.id" toString == "{{timeSpentID}}"
jsonpath "$.activity_id" toString == "{{id}}"

# update activity second time
PUT {{host}}/{{version}}/activity/{{id}}
Authorization: {{token}}

{
    "title": "{{title}}",
    "content": "{{content}}",
    "time": {{time}}
}

HTTP 200

[Asserts]
jsonpath "$.id" toString == {{id}}

# check time spent exist

GET {{host}}/{{version}}/time_spent/{{timeSpentID}}
Authorization: {{token}}

HTTP 200

[Asserts]
jsonpath "$.id" toString == "{{timeSpentID}}"
jsonpath "$.activity_id" toString == "{{id}}"

# Delete activity
DELETE {{host}}/{{version}}/activity/{{id}}
Authorization: {{token}}

HTTP 200

[Asserts]
body contains {{id}}


# time spent
POST {{sql_query_path}}

"select title from activities where account_id={{accountID}};"

HTTP 200

[Asserts]
body == "[]"

POST {{host}}/{{version}}/time_spent
Authorization: {{token}}

{
  "time": {{time}},
  "activity_id": {{id}}
}

HTTP 422

[Asserts]
body == "Cannot update data"










