// Helpers puros para manipular o bloco de frontmatter `--- ... ---`
// em arquivos .md. Sem dependências — só manipulação de texto.

/// Separa o bloco de frontmatter do corpo. Retorna `(bloco_interno, corpo)`.
/// Conteúdo sem delimitador `---` inicial fechado é tratado como sem frontmatter.
pub fn split_frontmatter(content: &str) -> (Option<String>, String) {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---\n") {
        return (None, content.to_string());
    }
    let after_open = &trimmed[4..];
    if let Some(close_pos) = after_open.find("\n---") {
        let fm = after_open[..close_pos].trim_end().to_string();
        let rest = &after_open[close_pos + 4..];
        let body = rest.strip_prefix('\n').unwrap_or(rest).to_string();
        (Some(fm), body)
    } else {
        (None, content.to_string())
    }
}

/// Insere ou substitui a linha `title:` dentro do texto do bloco,
/// preservando as demais linhas.
pub fn set_title_in_fm(fm: Option<&str>, title: &str) -> String {
    let title_line = format!("title: {title}");
    match fm {
        None => title_line,
        Some(text) => {
            if text.lines().any(|l| l.trim_start().starts_with("title:")) {
                text.lines()
                    .map(|l| {
                        if l.trim_start().starts_with("title:") {
                            title_line.clone()
                        } else {
                            l.to_string()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            } else {
                format!("{title_line}\n{text}")
            }
        }
    }
}

/// Monta o conteúdo completo do arquivo: bloco frontmatter + corpo.
pub fn compose_document(fm_text: Option<&str>, body: &str) -> String {
    match fm_text {
        None => body.to_string(),
        Some(fm) => format!("---\n{fm}\n---\n{body}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_sem_frontmatter_retorna_corpo_inteiro() {
        let (fm, body) = split_frontmatter("# Nota\n\nconteudo");
        assert!(fm.is_none());
        assert_eq!(body, "# Nota\n\nconteudo");
    }

    #[test]
    fn split_com_frontmatter_separa_bloco_e_corpo() {
        let content = "---\ntitle: Minha Nota\n---\n\n# Nota\n";
        let (fm, body) = split_frontmatter(content);
        assert_eq!(fm.as_deref(), Some("title: Minha Nota"));
        assert_eq!(body, "\n# Nota\n");
    }

    #[test]
    fn split_com_delimitador_nao_fechado_trata_como_sem_fm() {
        let (fm, body) = split_frontmatter("---\ntitle: sem fechar");
        assert!(fm.is_none());
        assert_eq!(body, "---\ntitle: sem fechar");
    }

    #[test]
    fn set_title_insere_em_fm_inexistente() {
        assert_eq!(set_title_in_fm(None, "Nova"), "title: Nova");
    }

    #[test]
    fn set_title_substitui_linha_existente_preservando_outras() {
        let fm = "tags: a,b\ntitle: Antiga\nstatus: ativo";
        assert_eq!(
            set_title_in_fm(Some(fm), "Nova"),
            "tags: a,b\ntitle: Nova\nstatus: ativo"
        );
    }

    #[test]
    fn compose_sem_fm_retorna_só_corpo() {
        assert_eq!(compose_document(None, "# corpo"), "# corpo");
    }

    #[test]
    fn compose_com_fm_produz_bloco_e_roundtrip_com_split() {
        let composed = compose_document(Some("title: X"), "# corpo\n");
        let (fm, body) = split_frontmatter(&composed);
        assert_eq!(fm.as_deref(), Some("title: X"));
        assert_eq!(body, "# corpo\n");
    }
}
