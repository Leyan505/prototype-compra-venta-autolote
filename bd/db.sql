DROP TABLE venta;
DROP TABLE promesa_venta;
DROP TABLE cliente;
DROP TABLE gasto;
DROP TABLE vehiculo;
DROP TABLE vendedor;

CREATE TABLE Vehiculo (
    nro_chasis VARCHAR(50) UNIQUE NOT NULL,
    matricula VARCHAR(15) PRIMARY KEY,
    modelo VARCHAR(50) NOT NULL,
    marca VARCHAR(50) NOT NULL,
    color VARCHAR(30),
    anio INT NOT NULL,
    fecha_compra DATE NOT NULL,
    precio_compra NUMERIC(12,2) NOT NULL
);

CREATE TABLE Gasto (
    id_gasto SERIAL PRIMARY KEY,
    matricula VARCHAR(15) NOT NULL,
    tipo_reparacion VARCHAR(100) NOT NULL,
    monto NUMERIC(12, 2) NOT NULL,
    fecha_finalizacion DATE NOT NULL,
    nombre_taller VARCHAR(50) NOT NULL,
    direccion_taller VARCHAR(100),
    telefono_taller VARCHAR(20),
    FOREIGN KEY (matricula) REFERENCES Vehiculo (matricula)
);


CREATE TABLE Cliente (
    id_cliente SERIAL PRIMARY KEY,
    nombre VARCHAR(50) NOT NULL,
    apellido VARCHAR(50) NOT NULL,
    cedula VARCHAR(16) UNIQUE NOT NULL
);


CREATE TABLE Vendedor (
    id_vendedor SERIAL PRIMARY KEY,
    nombre VARCHAR(50) NOT NULL,
    apellido VARCHAR(50) NOT NULL,
    cedula VARCHAR(16) UNIQUE NOT NULL
);


CREATE TABLE Venta (
    id_venta SERIAL PRIMARY KEY,
    matricula VARCHAR(15) NOT NULL,
    fecha_venta DATE NOT NULL,
    precio_venta NUMERIC(12, 2) NOT NULL,
    id_cliente INT NOT NULL,
    id_vendedor INT NOT NULL,
    FOREIGN KEY (matricula) REFERENCES Vehiculo (matricula),
    FOREIGN KEY (id_cliente) REFERENCES Cliente (id_cliente),
    FOREIGN KEY (id_vendedor) REFERENCES Vendedor (id_vendedor)
);


CREATE TABLE Promesa_Venta (
    id_promesa SERIAL PRIMARY KEY,
    id_cliente INT NOT NULL,
    matricula VARCHAR(15) NOT NULL,
    monto_promesa NUMERIC(12, 2) NOT NULL,
    estado VARCHAR(50) NOT NULL,
    fecha_realizacion DATE NOT NULL,
    fecha_finalizacion DATE,
    FOREIGN KEY (id_cliente) REFERENCES Cliente (id_cliente),
    FOREIGN KEY (matricula) REFERENCES Vehiculo (matricula)
);

CREATE TYPE state AS ENUM('DISPONIBLE', 'ENPROCESO', 'VENDIDO', 'RESERVADO');

ALTER TABLE vehiculo ADD COLUMN estado state;


CREATE TYPE estado AS ENUM('ACTIVO', 'OUT');
ALTER TABLE vendedor ADD COLUMN estado estado;