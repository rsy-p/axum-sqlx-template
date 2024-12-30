-- Add migration script here
DROP TABLE IF EXISTS "user";

CREATE TABLE "public"."user"
(
    "id"   uuid                                        NOT NULL,
    "name" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
    CONSTRAINT "user_pkey" PRIMARY KEY ("id")
);