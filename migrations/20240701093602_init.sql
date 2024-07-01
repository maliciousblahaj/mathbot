CREATE TABLE "Accounts" (
	"id"	        INTEGER, 
	"user_id"	    INTEGER NOT NULL UNIQUE, --discord userid
	"created"	    INTEGER NOT NULL, --unix timestamp
	"balance"	    REAL NOT NULL DEFAULT 100, 
	"smpssolved"	INTEGER NOT NULL DEFAULT 0, --simple math problems solved
	"isbanned"	    INTEGER NOT NULL DEFAULT 0, --bool
	"mineslots"	    INTEGER NOT NULL DEFAULT 0, --number of mine slots
	"previousclaim"	INTEGER NOT NULL DEFAULT 0, --unix timestamp
	"awaitingclaim"	INTEGER NOT NULL DEFAULT 0, --number of mathcoins that have not been claimed
	"username"		TEXT NOT NULL UNIQUE,
	"userbio"		TEXT,
	"pronouns"		TEXT,
	"avatarurl"		TEXT NOT NULL,
	"nextusernameupdate"	INTEGER NOT NULL DEFAULT 0, --unix timestamp
	"isadmin"		INTEGER NOT NULL DEFAULT 0, --bool
	PRIMARY KEY("id")
)

CREATE TABLE "Inventory" (
	"db_id"			INTEGER NOT NULL,
	"item_id"		INTEGER NOT NULL,
	"count"			INTEGER NOT NULL DEFAULT 0,
	FOREIGN KEY("item_id") REFERENCES "Items"("id"),
	FOREIGN KEY("db_id") REFERENCES "Accounts"("id")
)

CREATE TABLE "Items" (
        "id"    	INTEGER,
        "name_id"    TEXT NOT NULL,
        "emoji_id"   TEXT,
        "imageurl"  TEXT,
        "name"  	TEXT NOT NULL,
        "itemtype"  TEXT,
        "forsale"   INTEGER,
        "price" 	INTEGER,
        "description"   TEXT,
        "multiplier"    REAL,
        "mps"   	REAL,
        PRIMARY KEY("id")
)

CREATE TABLE "Slots" (
	"id"			INTEGER,
	"db_id"			INTEGER NOT NULL,
	"item_id"		INTEGER NOT NULL DEFAULT 0,
	PRIMARY KEY("id"),
	FOREIGN KEY("db_id") REFERENCES "Accounts"("id"),
	FOREIGN KEY("item_id") REFERENCES "Items"("id")
)