CREATE TABLE IF NOT EXISTS "Accounts" (
	"id"	        INTEGER PRIMARY KEY NOT NULL, 
	"user_id"	    INTEGER NOT NULL UNIQUE, --discord userid
	"created"	    INTEGER NOT NULL, --unix timestamp
	"balance"	    REAL NOT NULL DEFAULT 100, 
	"smps_solved"	INTEGER NOT NULL DEFAULT 0, --simple math problems solved
	"banned"	    INTEGER NOT NULL DEFAULT 0,
	"mine_slots"	INTEGER NOT NULL DEFAULT 0, --number of mine slots
	"previous_claim"	INTEGER NOT NULL DEFAULT 0, --unix timestamp
	"awaiting_claim"	INTEGER NOT NULL DEFAULT 0, --number of mathcoins that have not been claimed
	"username"		TEXT NOT NULL COLLATE NOCASE UNIQUE,
	"user_bio"		TEXT, --optional
	"pronouns"		TEXT, --optional
	"avatar_url"	TEXT NOT NULL,
	"next_username_update"	INTEGER NOT NULL DEFAULT 0, --unix timestamp
	"last_election_vote"	TEXT, --last candidate voted
	"is_admin"		INTEGER NOT NULL DEFAULT 0 --bool
);

CREATE TABLE IF NOT EXISTS "Inventory" (
	"account_id"	INTEGER NOT NULL,
	"item_id"		INTEGER NOT NULL,
	"count"			INTEGER NOT NULL DEFAULT 0,
	UNIQUE("account_id", "item_id"),
	FOREIGN KEY("account_id") REFERENCES "Accounts"("id"),
	FOREIGN KEY("item_id") REFERENCES "Items"("id")
);

CREATE TABLE IF NOT EXISTS "Items" (
        "id"    		INTEGER PRIMARY KEY NOT NULL,
        "name_id"   	TEXT NOT NULL COLLATE NOCASE UNIQUE,
        "emoji_id"  	TEXT, --optional
        "image_url" 	TEXT, --optional
        "display_name" 	TEXT NOT NULL,
        "item_type" 	TEXT NOT NULL,
        "price" 		INTEGER, --if it's null then it's not for sale
        "description"   TEXT, --optional
        "multiplier"    REAL, --optional
        "mps"   		REAL --optional
);

CREATE TABLE IF NOT EXISTS "Slots" (
	"id"			INTEGER PRIMARY KEY NOT NULL,
	"account_id"	INTEGER NOT NULL,
	"item_id"		INTEGER,
	FOREIGN KEY("account_id") REFERENCES "Accounts"("id"),
	FOREIGN KEY("item_id") REFERENCES "Items"("id")
);