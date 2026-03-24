import { useEffect, useState } from "react";
import { fetchCharacters } from "../api/characters";
import type { Character } from "../types/character";

export default function CharacterList() {
  const [characters, setCharacters] = useState<Character[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetchCharacters()
      .then(data => {
        setCharacters(data);
      })
      .catch(err => {
        setError(err.message);
      })
      .finally(() => setLoading(false));
  }, []);

  if (loading) return <div>Loading...</div>;
  if (error) return <div>Error: {error}</div>;

  return (
    <ul>
      {characters.map(c => (
        <li key={c.index}>
          ID {c.index} – Name {c.name} – Element {c.element} – Style {c.style}
        </li>
      ))}
    </ul>
  );
}
