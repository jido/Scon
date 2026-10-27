# Simple compact object notation

The Scon format is based on minified JSON with the noise removed, which makes it suitable for efficient transport and for machine parsing while remaining somewhat intelligible to humans.

---

## Syntax

### Separators

The comma `,` and the newline are interchangeable in Scon.

> [!NOTE]
> it means that a comma cannot be followed by a newline, in the same way that a comma cannot be followed by another comma in JSON.

### Objects

An object begins with a curly brace `{` and ends with a curly brace `}`.

A newline is allowed after `{` and there can be a comma or newline and spaces before `}` (but that increases the size).

### Keys

The keys of an object are written without quotes. A key name must not contain spaces or other significant characters.

```
somekey
```

A key name with spaces or significant characters can be written inside single quotes `'`. Escapes must be rendered to UTF-8; `\` represents a backslash and does not introduce an escaped character. A single quote in the key must be doubled `''`.

Spaces are allowed before the key (but that increases the size).

#### Shorthand

A key can be associated with a shorthand name which must not contain spaces or other significant characters. To associate a key with a shorthand, write the short name followed by the long name separated with a colon `:`:

```
ab:'some key'
```

From this point on, the long name can be replaced with the shorthand when writing a key name.

### Lists

A list begins with a square bracket `[` and ends with a square bracket `]`.

A newline is allowed after `[` and there can be a comma or newline before `]` (but that increases the size).

> [!NOTE]
> List items don't start with a key, which means spaces are not allowed before an item (a single space has a special meaning).

### Text

There are three notations for text data in Scon.

#### Simple string

A simple string begins with a single space. It should not contain spaces, commas, `]` or `}`.

```
 simple-string
```

#### Long string

A long string begins and ends with a double quote `"`. Escapes must be rendered to UTF-8; `\` represents a backslash and does not introduce an escaped character. A double quote in the text must be doubled `""`.

```
"Text with spaces, commas and special characters such as ""\""."
```

Newlines are allowed in a long string.

#### Date string

A date string begins with a hat `^` and is written in ISO-8601 format.

```
^2026-10-26
```

### Number

A positive number begins with plus `+` and a negative number begins with minus `-`. Other than that the notation is the same as JSON.

```
+1.347e9
```

### Boolean

True is represented with an asterisk `*` and false with a tilde `~`.

### Null

Null is represented with a forward slash `/`

---

## An example

```clojure
[{
 id:userId+1
 Un:username"johndoe"
 Em:email"johndoe@example.com"
 hash:passwordHash"abc123hashedpassword"
 pr:profile{
  Fn:firstName"John"
  Ln:lastName"Doe"
  bd:birthDate^1990-01-01
  ge:gender male
  ava:avatarUrl https://example.com/avatars/johndoe.jpg
  bio"Software developer with a passion for open-source projects."
 }
 pref:preferences{
  lang:language en
  th:theme dark
  notif:notifications{Em*,sms~,push*}
  priv:privacy{
   onl:showOnlineStatus~
   vis:profileVisibility friends
  }
 }
 stat:accountStatus{
  activ:isActive*
  ll:lastLogin^2025-01-10T12:45:00Z
  cr:createdAt^2022-05-15T08:30:00Z
 }
 log:activityLogs[{
  ts:timestamp^2025-01-09T14:20:00Z
  ac:activity"Logged in from IP 192.168.1.10"
 },{
  ts^2025-01-08T10:05:00Z
  ac"Updated profile picture"
 }]
}
{id+2,Un"janedoe",Em"janedoe@example.com",hash"xyz987hashedpassword"
pr{Fn"Jane",Ln"Doe",bd^1992-04-12,ge female,ava https://example.com/avatars/janedoe.jpg
bio"Digital marketer and blogger."}
pref{lang fr,th light,notif{Em~,sms*,push*},priv{onl*,vis public}}
stat{activ*,ll^2025-01-10T09:15:00Z,cr^2023-03-20T11:00:00Z}
log[{ts^2025-01-09T18:30:00Z,ac"Commented on a post"}
{ts^2025-01-08T16:45:00Z,ac"Liked a post"}]}
]
```

It is derived from this sample JSON document:

```json
[
  {
    "userId": 1,
    "username": "johndoe",
    "email": "johndoe@example.com",
    "passwordHash": "abc123hashedpassword",
    "profile": {
      "firstName": "John",
      "lastName": "Doe",
      "birthDate": "1990-01-01",
      "gender": "male",
      "avatarUrl": "https://example.com/avatars/johndoe.jpg",
      "bio": "Software developer with a passion for open-source projects."
    },
    "preferences": {
      "language": "en",
      "theme": "dark",
      "notifications": {
        "email": true,
        "sms": false,
        "push": true
      },
      "privacy": {
        "showOnlineStatus": false,
        "profileVisibility": "friends"
      }
    },
    "accountStatus": {
      "isActive": true,
      "lastLogin": "2025-01-10T12:45:00Z",
      "createdAt": "2022-05-15T08:30:00Z"
    },
    "activityLogs": [
      {
        "timestamp": "2025-01-09T14:20:00Z",
        "activity": "Logged in from IP 192.168.1.10"
      },
      {
        "timestamp": "2025-01-08T10:05:00Z",
        "activity": "Updated profile picture"
      }
    ]
  },
  {
    "userId": 2,
    "username": "janedoe",
    "email": "janedoe@example.com",
    "passwordHash": "xyz987hashedpassword",
    "profile": {
      "firstName": "Jane",
      "lastName": "Doe",
      "birthDate": "1992-04-12",
      "gender": "female",
      "avatarUrl": "https://example.com/avatars/janedoe.jpg",
      "bio": "Digital marketer and blogger."
    },
    "preferences": {
      "language": "fr",
      "theme": "light",
      "notifications": {
        "email": false,
        "sms": true,
        "push": true
      },
      "privacy": {
        "showOnlineStatus": true,
        "profileVisibility": "public"
      }
    },
    "accountStatus": {
      "isActive": true,
      "lastLogin": "2025-01-10T09:15:00Z",
      "createdAt": "2023-03-20T11:00:00Z"
    },
    "activityLogs": [
      {
        "timestamp": "2025-01-09T18:30:00Z",
        "activity": "Commented on a post"
      },
      {
        "timestamp": "2025-01-08T16:45:00Z",
        "activity": "Liked a post"
      }
    ]
  }
]
```

Compare with the JSON minified version:

> _`[{"userId":1,"username":"johndoe","email":"johndoe@example.com","passwordHash":"abc123hashedpassword","profile":{"firstName":"John","lastName":"Doe","birthDate":"1990-01-01","gender":"male","avatarUrl":"https://example.com/avatars/johndoe.jpg","bio":"Software developer with a passion for open-source projects."},"preferences":{"language":"en","theme":"dark","notifications":{"email":true,"sms":false,"push":true},"privacy":{"showOnlineStatus":false,"profileVisibility":"friends"}},"accountStatus":{"isActive":true,"lastLogin":"2025-01-10T12:45:00Z","createdAt":"2022-05-15T08:30:00Z"},"activityLogs":[{"timestamp":"2025-01-09T14:20:00Z","activity":"Logged in from IP 192.168.1.10"},{"timestamp":"2025-01-08T10:05:00Z","activity":"Updated profile picture"}]},{"userId":2,"username":"janedoe","email":"janedoe@example.com","passwordHash":"xyz987hashedpassword","profile":{"firstName":"Jane","lastName":"Doe","birthDate":"1992-04-12","gender":"female","avatarUrl":"https://example.com/avatars/janedoe.jpg","bio":"Digital marketer and blogger."},"preferences":{"language":"fr","theme":"light","notifications":{"email":false,"sms":true,"push":true},"privacy":{"showOnlineStatus":true,"profileVisibility":"public"}},"accountStatus":{"isActive":true,"lastLogin":"2025-01-10T09:15:00Z","createdAt":"2023-03-20T11:00:00Z"},"activityLogs":[{"timestamp":"2025-01-09T18:30:00Z","activity":"Commented on a post"},{"timestamp":"2025-01-08T16:45:00Z","activity":"Liked a post"}]}]`_
