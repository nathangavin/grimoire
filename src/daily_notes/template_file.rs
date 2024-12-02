use std::io;

pub struct TemplateFile {
    file_path: String,
    content: Option<String>
}

impl TemplateFile {
    pub fn generate_template_file(&self) -> io::Result<()> {

        // check if file exists
        // create a new file at file location
        // populate file with content
        
        let sample_task = "- [ ] sample task";
        let top_line = "{TITLE} - {DATE}";
        let name_line = "{NAME}";
        let startup_tasks = "# Startup Tasks";
        let finishup_tasks = "# Finishup Tasks";
        let goals_for_today = "# Goals for Today";
        let small_tasks = "# Small tasks to do";
        let notes_today = "# Notes on Today";

        Ok(())
    }

    pub fn retrieve_template_file() -> TemplateFile {
           
    }
}
