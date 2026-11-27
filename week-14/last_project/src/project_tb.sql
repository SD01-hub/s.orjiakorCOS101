--
-- PostgreSQL database dump
--

\restrict BZIZblXbGu59qrh7FYaMOM3NzabFlJHOdgt80L9Qaiu7Yi7ImzmxTgkzy7sZNvq

-- Dumped from database version 18.7
-- Dumped by pg_dump version 18.7

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    project_no integer,
    project_name character varying(30),
    project_duration character varying(30),
    project_manager_id integer
);


ALTER TABLE public.project OWNER TO postgres;

--
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (project_no, project_name, project_duration, project_manager_id) FROM stdin;
11	A	9 Months	102
22	B	14 Months	97
33	C	16 Months	120
44	D	25 Months	108
55	E	9 Months	107
\.


--
-- PostgreSQL database dump complete
--

\unrestrict BZIZblXbGu59qrh7FYaMOM3NzabFlJHOdgt80L9Qaiu7Yi7ImzmxTgkzy7sZNvq

