<script lang="ts">
	import { marked } from 'marked';

	let { response }: { response: string } = $props();

	let answer = `Okay, let's dive into Maple Leaf International School! I’ve been analyzing your provided information and here’s a detailed overview of 
the school, designed to be helpful and engaging for a potential student or parent:

**Maple Leaf International School: A Legacy of Excellence**

Maple Leaf International School is a truly distinguished institution, steeped in history and driven by a commitment to nurturing 
well-rounded individuals. Founded in 1972, it’s one of the most prestigious English Medium schools in Bangladesh, boasting a long and 
proud tradition of academic excellence.  It’s more than just a school; it’s a community built on a foundation of values – intellectual 
curiosity, artistic expression, and practical skills. 

**Here’s a breakdown of what makes Maple Leaf special:**

**1. A Rich History & Tradition:**

* **Established in 1972:**  This longevity speaks volumes about the school’s dedication to continuous improvement and quality 
education.
* **British Curriculum:** It adheres to the British Curriculum, ensuring a robust foundation for IGCSE, GCE ‘A’ Levels, and 
International Advanced Level studies.
* **Accreditation:** Recognized by the Government of Bangladesh, Edexcel, and CIE – a testament to its rigorous standards and 
commitment to global recognition. 

**2. Academic Excellence – A Broad Spectrum:**

* **Extensive Curriculum:** The school offers a fantastic range of subjects covering natural science, liberal arts (Accounting, Art & 
Design, Bangla, Biology, Chemistry, Business Studies, Computing Studies, Economics, English Language & Literature, English Literature, 
Further Mathematics, Geography, German, Government & Politics, History, Law, Mathematics, Physics, Islamic Studies, Religious Studies, 
Bangladesh Studies.** This broad curriculum caters to diverse interests and learning styles.
* **State-of-the-Art Facilities:**  The school invests heavily in its facilities, including modern laboratories, multimedia centers, 
and a well-stocked library with over 10,000 books, encouraging reading habits. 
* **Focus on Specialization:** The school emphasizes practical skills and offers specialized programs like the "Faress Bhuiyan" who 
achieved 10 A grades in one sitting! 

**3. A Supportive & Engaging Environment:**

* **Dedicated Faculty:**  The school's teachers are highly qualified, experienced, and passionate about education.
* **Competitive Atmosphere:**  Maple Leaf fosters a competitive environment, but it's one that encourages students to excel and reach 
their full potential.
* **Extracurricular Activities:** This is where the real heart of the school shines. The school strongly believes in a well-rounded 
education and actively promotes cultural activities –  they have dedicated time for cultural activities, a vital part of the 
curriculum. 

**4.  Key Highlights & Achievements:**

* **National & International Recognition:**  The school consistently achieves outstanding results in both IGCSE and International 
Advanced Level exams, demonstrating their ability to prepare students for success.
* **Scholarship Opportunities:** A large percentage of students receive scholarships, signifying their dedication to achieving their 
academic goals. 
* **Curriculum Updates:** The curriculum is regularly updated to ensure it aligns with the latest educational standards.
* **Career Guidance & Counselling:** They provide comprehensive career counselling, assisting students in exploring their options and 
securing opportunities.


**5.  Resources & Support:**

* **Website:** [https://mlis.edu.bd/](https://mlis.edu.bd/) -  This is your primary resource for all things Maple Leaf.
* **E-mail:** info@mlis.edu.bd -  For any specific inquiries or questions.
* **Mobile:** +8801810199274, +8801810199284, +8801743181060, +8801783699672 -  For urgent matters.
* **Partners:** British Council, Pearson, Edexcel, UCAS, Goethe Institut, The Duke of Edinburgh’s International Award, ACT Club, South 
Asia - showcasing the school’s commitment to global education.


**To help me tailor my response even further to your needs, could you tell me:**

*   What specifically are you interested in learning about Maple Leaf? (e.g., admission process, specific programs, student life, etc.)`;
</script>

<div class="prose prose-neutral dark:prose-invert w-full">
	{#await marked(response) then renderedResponse}
		{@html renderedResponse}
	{:catch error}
		<p>Error rendering markdown: {error.message}</p>
	{/await}
</div>
