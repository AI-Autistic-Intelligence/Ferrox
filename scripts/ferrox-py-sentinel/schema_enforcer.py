import json

class LlmSchemaEnforcer:
    """Enforces strict JSON schema validation on LLM agent outputs (Building LLM Applications Ch. 4)."""

    @staticmethod
    def enforce_schema(raw_response: str, required_keys: list[str]) -> dict:
        """Parses LLM response, strips markdown formatting, and validates required JSON keys."""
        cleaned = raw_response.strip()
        if cleaned.startswith("```json"):
            cleaned = cleaned[7:]
        if cleaned.endswith("```"):
            cleaned = cleaned[:-3]
        cleaned = cleaned.strip()

        try:
            parsed = json.loads(cleaned)
            if not isinstance(parsed, dict):
                return {"valid": False, "error": "LLM response is not a JSON object", "data": None}

            missing_keys = [key for key in required_keys if key not in parsed]
            if missing_keys:
                return {"valid": False, "error": f"Missing required keys: {missing_keys}", "data": None}

            return {"valid": True, "error": None, "data": parsed}
        except json.JSONDecodeError as e:
            return {"valid": False, "error": f"Invalid JSON format: {str(e)}", "data": None}

if __name__ == "__main__":
    llm_output = '```json\n{"action": "allow_access", "risk_score": 0.05}\n```'
    validated = LlmSchemaEnforcer.enforce_schema(llm_output, ["action", "risk_score"])
    print("Schema Enforcer Result:", validated)
