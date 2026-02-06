# Guia de Versionamento

## Objetivo

Definir a política de versionamento e compatibilidade.

## SemVer

Adotamos [SemVer](https://semver.org/):

- **MAJOR**: mudanças incompatíveis.
- **MINOR**: novas funcionalidades compatíveis.
- **PATCH**: correções compatíveis.

## Política

- Alterações na SPEC que quebram compatibilidade exigem MAJOR.
- Mudanças de CLI que quebram flags/saídas exigem MAJOR.
- Novos recursos compatíveis incrementam MINOR.
- Correções e ajustes de docs incrementam PATCH.

## Pré-1.0

Antes da 1.0, mudanças podem ocorrer com maior frequência. Ainda assim, manteremos SemVer e documentaremos breaking changes no CHANGELOG.
