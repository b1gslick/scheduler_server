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
jsonpath "$.time" == {{expected}}

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

HTTP 201

[Asserts]
jsonpath "$.id" toString == {{id}}

# check activity updated

GET {{host}}/{{version}}/activity?limit={{limit}}&offset={{offset}}
Authorization: {{token}}

HTTP 200

[Asserts]
jsonpath "$..title" contains {{new_title}}
jsonpath "$..content" contains {{new_content}}

# update activity second time
PUT {{host}}/{{version}}/activity/{{id}}
Authorization: {{token}}

{
    "title": "{{title}}",
    "content": "{{content}}",
    "time": {{time}}
}

HTTP 201

[Asserts]
jsonpath "$.id" toString == {{id}}

# Delete activity
DELETE {{host}}/{{version}}/activity/{{id}}
Authorization: {{token}}

HTTP 200

[Asserts]
body contains {{id}}
