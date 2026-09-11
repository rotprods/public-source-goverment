const capabilities = [
  "Expresar una necesidad en lenguaje cotidiano",
  "Entender qué institución o ámbito puede resolverla",
  "Encontrar necesidades y propuestas relacionadas",
  "Seguir evidencia, versiones, respuestas y compromisos",
];

export default function HomePage() {
  return (
    <main className="shell">
      <header className="hero">
        <p className="eyebrow">INFRAESTRUCTURA CÍVICA · BOOTSTRAP</p>
        <h1>De una necesidad pública a una respuesta trazable.</h1>
        <p className="lead">
          Esta interfaz es un shell de desarrollo. No representa todavía el producto final ni fija su nombre público.
        </p>
      </header>

      <section aria-labelledby="north-star" className="panel">
        <p className="eyebrow">NORTH STAR</p>
        <h2 id="north-star">Qualified Civic Loop Completion</h2>
        <p>
          Mediremos si una necesidad real llega a una resolución pública, versionada, competente y verificable — no cuánto tiempo permanece alguien en la app.
        </p>
      </section>

      <section aria-labelledby="first-slice" className="panel">
        <p className="eyebrow">PRIMER VERTICAL SLICE</p>
        <h2 id="first-slice">El ciudadano empieza hablando como ciudadano.</h2>
        <ul>
          {capabilities.map((capability) => (
            <li key={capability}>{capability}</li>
          ))}
        </ul>
      </section>
    </main>
  );
}
