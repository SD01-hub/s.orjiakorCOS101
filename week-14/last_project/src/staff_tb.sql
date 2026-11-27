--
-- PostgreSQL database dump
--

\restrict brfB45HUK9VxiDEvOn6SQIZBWjPS4rf9fLjtxCc0Oj9JVeiZo78yFs566OIlsOW

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
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer CONSTRAINT employees_id_not_null NOT NULL,
    staff_name character varying(30) NOT NULL,
    department_no integer NOT NULL,
    staff_sal character varying(20) NOT NULL,
    age integer NOT NULL,
    mobile character varying(15) NOT NULL,
    gender character varying(1)
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, department_no, staff_sal, age, mobile, gender) FROM stdin;
101	Alade Joy	2	$250,000.00	33	08023089832	\N
100	Mustapha Ali	3	$175,000.00	32	08063285831	\N
107	Alokwe Martin	7	$380,000.00	48	07090082812	\N
97	Dankade Aminat	5	$550,000.00	40	09023688832	\N
108	Josiah Joshua	1	$120,000.00	30	08053189131	\N
102	Mankinde Mary	2	$450,000.00	55	09023487830	\N
120	Adeleke Jane	4	$200,000.00	38	07061045862	\N
122	Osahon Mark	6	$320,000.00	44	08022289842	\N
117	Suleman Ajayi	3	$800,000.00	50	07030089811	\N
104	Kuti Lawal	1	$750,000.00	35	09145689842	\N
\.


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- PostgreSQL database dump complete
--

\unrestrict brfB45HUK9VxiDEvOn6SQIZBWjPS4rf9fLjtxCc0Oj9JVeiZo78yFs566OIlsOW

