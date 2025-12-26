using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Arxis.Infrastructure.Data;
using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.ChangeTracking;
using Microsoft.EntityFrameworkCore.Infrastructure;
using Microsoft.EntityFrameworkCore.Metadata;
using Microsoft.EntityFrameworkCore.Storage.ValueConversion;

#nullable disable

namespace Arxis.Infrastructure.Migrations;

[DbContext(typeof(ArxisDbContext))]
partial class ArxisDbContextModelSnapshot : ModelSnapshot
{
    protected override void BuildModel(ModelBuilder modelBuilder)
    {
#pragma warning disable 612, 618
        modelBuilder.HasAnnotation("ProductVersion", "8.0.11");

        var listStringConverter = new ValueConverter<List<string>, string>(
            v => JsonSerializer.Serialize(v ?? new List<string>(), default(JsonSerializerOptions)),
            v => string.IsNullOrWhiteSpace(v)
                ? new List<string>()
                : JsonSerializer.Deserialize<List<string>>(v, default(JsonSerializerOptions)) ?? new List<string>());

        var listStringComparer = new ValueComparer<List<string>>(
            (l, r) => (l == null && r == null) || (l != null && r != null && l.SequenceEqual(r)),
            v => v == null ? 0 : v.Aggregate(0, (current, element) => HashCode.Combine(current, element == null ? 0 : element.GetHashCode())),
            v => v == null ? new List<string>() : v.ToList());

        modelBuilder.Entity("Arxis.Domain.Entities.Project", b =>
            {
                b.Property<Guid>("Id")
                    .HasColumnType("TEXT");

                b.Property<string>("Address")
                    .HasColumnType("TEXT");

                b.Property<string>("City")
                    .HasColumnType("TEXT");

                b.Property<string>("Client")
                    .HasColumnType("TEXT");

                b.Property<DateTime?>("ContractDate")
                    .HasColumnType("TEXT");

                b.Property<string>("Country")
                    .HasColumnType("TEXT");

                b.Property<DateTime>("CreatedAt")
                    .HasColumnType("TEXT");

                b.Property<string>("CreatedBy")
                    .HasColumnType("TEXT");

                b.Property<string>("Currency")
                    .HasMaxLength(3)
                    .HasColumnType("TEXT");

                b.Property<string>("Description")
                    .HasColumnType("TEXT");

                b.Property<DateTime?>("EndDate")
                    .HasColumnType("TEXT");

                b.Property<bool>("IsDeleted")
                    .HasColumnType("INTEGER");

                b.Property<string>("Name")
                    .HasMaxLength(200)
                    .HasColumnType("TEXT");

                b.Property<string>("State")
                    .HasColumnType("TEXT");

                b.Property<DateTime?>("StartDate")
                    .HasColumnType("TEXT");

                b.Property<int>("Status")
                    .HasColumnType("INTEGER");

                b.Property<Guid?>("TenantId")
                    .HasColumnType("TEXT");

                b.Property<decimal?>("TotalBudget")
                    .HasPrecision(18, 2)
                    .HasColumnType("TEXT");

                b.Property<int>("Type")
                    .HasColumnType("INTEGER");

                b.Property<string>("UpdatedBy")
                    .HasColumnType("TEXT");

                b.Property<DateTime?>("UpdatedAt")
                    .HasColumnType("TEXT");

                b.Property<List<string>>("Tags")
                    .HasConversion(listStringConverter, listStringComparer, listStringComparer)
                    .HasColumnType("TEXT");

                b.HasKey("Id");

                b.HasIndex("TenantId");

                b.ToTable("Projects");
            });

        modelBuilder.Entity("Arxis.Domain.Entities.ProjectUser", b =>
            {
                b.Property<Guid>("Id")
                    .HasColumnType("TEXT");

                b.Property<string>("CreatedBy")
                    .HasColumnType("TEXT");

                b.Property<DateTime>("CreatedAt")
                    .HasColumnType("TEXT");

                b.Property<bool>("IsDeleted")
                    .HasColumnType("INTEGER");

                b.Property<Guid>("ProjectId")
                    .HasColumnType("TEXT");

                b.Property<int>("Role")
                    .HasColumnType("INTEGER");

                b.Property<Guid>("UserId")
                    .HasColumnType("TEXT");

                b.Property<string>("UpdatedBy")
                    .HasColumnType("TEXT");

                b.Property<DateTime?>("UpdatedAt")
                    .HasColumnType("TEXT");

                b.HasKey("Id");

                b.HasIndex("UserId");

                b.HasIndex("ProjectId", "UserId")
                    .IsUnique();

                b.ToTable("ProjectUsers");
            });

        modelBuilder.Entity("Arxis.Domain.Entities.User", b =>
            {
                b.Property<Guid>("Id")
                    .HasColumnType("TEXT");

                b.Property<string>("Avatar")
                    .HasColumnType("TEXT");

                b.Property<string>("CreatedBy")
                    .HasColumnType("TEXT");

                b.Property<DateTime>("CreatedAt")
                    .HasColumnType("TEXT");

                b.Property<string>("Email")
                    .HasMaxLength(255)
                    .HasColumnType("TEXT");

                b.Property<string>("FirstName")
                    .HasMaxLength(100)
                    .HasColumnType("TEXT");

                b.Property<bool>("IsActive")
                    .HasColumnType("INTEGER");

                b.Property<bool>("IsDeleted")
                    .HasColumnType("INTEGER");

                b.Property<string>("Language")
                    .HasColumnType("TEXT");

                b.Property<DateTime?>("LastLoginAt")
                    .HasColumnType("TEXT");

                b.Property<string>("LastName")
                    .HasMaxLength(100)
                    .HasColumnType("TEXT");

                b.Property<string>("PasswordHash")
                    .HasColumnType("TEXT");

                b.Property<string>("Phone")
                    .HasColumnType("TEXT");

                b.Property<string>("Role")
                    .HasColumnType("TEXT");

                b.Property<Guid?>("TenantId")
                    .HasColumnType("TEXT");

                b.Property<string>("UpdatedBy")
                    .HasColumnType("TEXT");

                b.Property<DateTime?>("UpdatedAt")
                    .HasColumnType("TEXT");

                b.HasKey("Id");

                b.HasIndex("Email");

                b.HasIndex("TenantId");

                b.ToTable("Users");
            });

        modelBuilder.Entity("Arxis.Domain.Entities.WorkTask", b =>
            {
                b.Property<Guid>("Id")
                    .HasColumnType("TEXT");

                b.Property<Guid?>("AssignedToUserId")
                    .HasColumnType("TEXT");

                b.Property<DateTime?>("CompletedAt")
                    .HasColumnType("TEXT");

                b.Property<string>("CreatedBy")
                    .HasColumnType("TEXT");

                b.Property<DateTime>("CreatedAt")
                    .HasColumnType("TEXT");

                b.Property<string>("Description")
                    .HasColumnType("TEXT");

                b.Property<DateTime?>("DueDate")
                    .HasColumnType("TEXT");

                b.Property<bool>("IsDeleted")
                    .HasColumnType("INTEGER");

                b.Property<Guid?>("ParentTaskId")
                    .HasColumnType("TEXT");

                b.Property<int>("Priority")
                    .HasColumnType("INTEGER");

                b.Property<Guid>("ProjectId")
                    .HasColumnType("TEXT");

                b.Property<int>("Status")
                    .HasColumnType("INTEGER");

                b.Property<string>("Title")
                    .HasMaxLength(300)
                    .HasColumnType("TEXT");

                b.Property<string>("UpdatedBy")
                    .HasColumnType("TEXT");

                b.Property<DateTime?>("UpdatedAt")
                    .HasColumnType("TEXT");

                b.Property<List<string>>("Tags")
                    .HasConversion(listStringConverter, listStringComparer, listStringComparer)
                    .HasColumnType("TEXT");

                b.HasKey("Id");

                b.HasIndex("AssignedToUserId");

                b.HasIndex("ParentTaskId");

                b.HasIndex("ProjectId");

                b.ToTable("WorkTasks");
            });

        modelBuilder.Entity("Arxis.Domain.Entities.Issue", b =>
            {
                b.Property<Guid>("Id")
                    .HasColumnType("TEXT");

                b.Property<Guid?>("AssignedToUserId")
                    .HasColumnType("TEXT");

                b.Property<DateTime>("CreatedAt")
                    .HasColumnType("TEXT");

                b.Property<string>("CreatedBy")
                    .HasColumnType("TEXT");

                b.Property<string>("Description")
                    .HasColumnType("TEXT");

                b.Property<string>("Discipline")
                    .HasMaxLength(120)
                    .HasColumnType("TEXT");

                b.Property<DateTime?>("DueDate")
                    .HasColumnType("TEXT");

                b.Property<string>("ExternalReference")
                    .HasMaxLength(200)
                    .HasColumnType("TEXT");

                b.Property<bool>("IsBlocking")
                    .HasColumnType("INTEGER");

                b.Property<bool>("IsDeleted")
                    .HasColumnType("INTEGER");

                b.Property<bool>("IsRFI")
                    .HasColumnType("INTEGER");

                b.Property<string>("Location")
                    .HasMaxLength(200)
                    .HasColumnType("TEXT");

                b.Property<int>("Priority")
                    .HasColumnType("INTEGER");

                b.Property<Guid>("ProjectId")
                    .HasColumnType("TEXT");

                b.Property<Guid?>("ReportedByUserId")
                    .HasColumnType("TEXT");

                b.Property<DateTime?>("RespondedAt")
                    .HasColumnType("TEXT");

                b.Property<Guid?>("RespondedByUserId")
                    .HasColumnType("TEXT");

                b.Property<DateTime?>("ResponseDueDate")
                    .HasColumnType("TEXT");

                b.Property<string>("Resolution")
                    .HasColumnType("TEXT");

                b.Property<DateTime?>("ResolvedAt")
                    .HasColumnType("TEXT");

                b.Property<string>("ReferenceCode")
                    .HasMaxLength(40)
                    .HasColumnType("TEXT");

                b.Property<string>("RfiAnswer")
                    .HasMaxLength(2000)
                    .HasColumnType("TEXT");

                b.Property<string>("RfiQuestion")
                    .HasMaxLength(2000)
                    .HasColumnType("TEXT");

                b.Property<int?>("SlaMinutes")
                    .HasColumnType("INTEGER");

                b.Property<int>("Status")
                    .HasColumnType("INTEGER");

                b.Property<int>("Type")
                    .HasColumnType("INTEGER");

                b.Property<string>("Title")
                    .HasMaxLength(300)
                    .HasColumnType("TEXT");

                b.Property<string>("UpdatedBy")
                    .HasColumnType("TEXT");

                b.Property<DateTime?>("UpdatedAt")
                    .HasColumnType("TEXT");

                b.Property<Guid?>("WorkTaskId")
                    .HasColumnType("TEXT");

                b.HasKey("Id");

                b.HasIndex("AssignedToUserId");

                b.HasIndex("ProjectId");

                b.HasIndex("ReportedByUserId");

                b.HasIndex("RespondedByUserId");

                b.HasIndex("WorkTaskId");

                b.HasIndex("ProjectId", "ReferenceCode")
                    .IsUnique();

                b.ToTable("Issues");
            });

        modelBuilder.Entity("Arxis.Domain.Entities.IssueAttachment", b =>
            {
                b.Property<Guid>("Id")
                    .HasColumnType("TEXT");

                b.Property<Guid?>("CommentId")
                    .HasColumnType("TEXT");

                b.Property<string>("ContentType")
                    .HasMaxLength(120)
                    .HasColumnType("TEXT");

                b.Property<DateTime>("CreatedAt")
                    .HasColumnType("TEXT");

                b.Property<string>("CreatedBy")
                    .HasColumnType("TEXT");

                b.Property<bool>("IsDeleted")
                    .HasColumnType("INTEGER");

                b.Property<Guid>("IssueId")
                    .HasColumnType("TEXT");

                b.Property<string>("FileName")
                    .HasMaxLength(260)
                    .HasColumnType("TEXT");

                b.Property<string>("FileUrl")
                    .HasMaxLength(500)
                    .HasColumnType("TEXT");

                b.Property<long>("FileSize")
                    .HasColumnType("INTEGER");

                b.Property<Guid?>("UploadedByUserId")
                    .HasColumnType("TEXT");

                b.Property<DateTime?>("UpdatedAt")
                    .HasColumnType("TEXT");

                b.Property<string>("UpdatedBy")
                    .HasColumnType("TEXT");

                b.HasKey("Id");

                b.HasIndex("CommentId");

                b.HasIndex("IssueId");

                b.HasIndex("UploadedByUserId");

                b.ToTable("IssueAttachments");
            });

        modelBuilder.Entity("Arxis.Domain.Entities.IssueComment", b =>
            {
                b.Property<Guid>("Id")
                    .HasColumnType("TEXT");

                b.Property<Guid?>("AuthorId")
                    .HasColumnType("TEXT");

                b.Property<DateTime>("CreatedAt")
                    .HasColumnType("TEXT");

                b.Property<string>("CreatedBy")
                    .HasColumnType("TEXT");

                b.Property<bool>("IsDeleted")
                    .HasColumnType("INTEGER");

                b.Property<bool>("IsInternal")
                    .HasColumnType("INTEGER");

                b.Property<Guid>("IssueId")
                    .HasColumnType("TEXT");

                b.Property<string>("Message")
                    .HasMaxLength(4000)
                    .HasColumnType("TEXT");

                b.Property<string>("UpdatedBy")
                    .HasColumnType("TEXT");

                b.Property<DateTime?>("UpdatedAt")
                    .HasColumnType("TEXT");

                b.HasKey("Id");

                b.HasIndex("AuthorId");

                b.HasIndex("IssueId");

                b.ToTable("IssueComments");
            });

        modelBuilder.Entity("Arxis.Domain.Entities.IssueLink", b =>
            {
                b.Property<Guid>("Id")
                    .HasColumnType("TEXT");

                b.Property<string>("CreatedBy")
                    .HasColumnType("TEXT");

                b.Property<DateTime>("CreatedAt")
                    .HasColumnType("TEXT");

                b.Property<string>("ExternalReference")
                    .HasMaxLength(200)
                    .HasColumnType("TEXT");

                b.Property<bool>("IsDeleted")
                    .HasColumnType("INTEGER");

                b.Property<Guid>("IssueId")
                    .HasColumnType("TEXT");

                b.Property<string>("Label")
                    .HasMaxLength(200)
                    .HasColumnType("TEXT");

                b.Property<int>("LinkType")
                    .HasColumnType("INTEGER");

                b.Property<string>("Metadata")
                    .HasMaxLength(2000)
                    .HasColumnType("TEXT");

                b.Property<Guid?>("RelatedEntityId")
                    .HasColumnType("TEXT");

                b.Property<string>("UpdatedBy")
                    .HasColumnType("TEXT");

                b.Property<DateTime?>("UpdatedAt")
                    .HasColumnType("TEXT");

                b.HasKey("Id");

                b.HasIndex("IssueId");

                b.ToTable("IssueLinks");
            });

        modelBuilder.Entity("Arxis.Domain.Entities.ProjectUser", b =>
            {
                b.HasOne("Arxis.Domain.Entities.Project", "Project")
                    .WithMany("ProjectUsers")
                    .HasForeignKey("ProjectId")
                    .OnDelete(DeleteBehavior.Cascade)
                    .IsRequired();

                b.HasOne("Arxis.Domain.Entities.User", "User")
                    .WithMany("ProjectUsers")
                    .HasForeignKey("UserId")
                    .OnDelete(DeleteBehavior.Cascade)
                    .IsRequired();

                b.Navigation("Project");

                b.Navigation("User");
            });

        modelBuilder.Entity("Arxis.Domain.Entities.WorkTask", b =>
            {
                b.HasOne("Arxis.Domain.Entities.User", "AssignedToUser")
                    .WithMany()
                    .HasForeignKey("AssignedToUserId")
                    .OnDelete(DeleteBehavior.SetNull);

                b.HasOne("Arxis.Domain.Entities.WorkTask", "ParentTask")
                    .WithMany("SubTasks")
                    .HasForeignKey("ParentTaskId")
                    .OnDelete(DeleteBehavior.Restrict);

                b.HasOne("Arxis.Domain.Entities.Project", "Project")
                    .WithMany()
                    .HasForeignKey("ProjectId")
                    .OnDelete(DeleteBehavior.Cascade)
                    .IsRequired();

                b.OwnsMany("Arxis.Domain.Entities.TaskChecklistItem", "Checklist", b1 =>
                    {
                        b1.Property<int>("Id")
                            .ValueGeneratedOnAdd()
                            .HasColumnType("INTEGER")
                            .HasAnnotation("Sqlite:Autoincrement", true);

                        b1.Property<bool>("IsCompleted")
                            .HasColumnType("INTEGER")
                            .HasDefaultValue(false);

                        b1.Property<Guid>("WorkTaskId")
                            .HasColumnType("TEXT");

                        b1.Property<string>("Title")
                            .HasMaxLength(200)
                            .HasColumnType("TEXT");

                        b1.HasKey("Id");

                        b1.HasIndex("WorkTaskId");

                        b1.ToTable("WorkTaskChecklistItems");

                        b1.WithOwner()
                            .HasForeignKey("WorkTaskId");
                    });

                b.Navigation("AssignedToUser");

                b.Navigation("Checklist");

                b.Navigation("ParentTask");

                b.Navigation("Project");

                b.Navigation("SubTasks");
            });

        modelBuilder.Entity("Arxis.Domain.Entities.Issue", b =>
            {
                b.HasOne("Arxis.Domain.Entities.User", "AssignedToUser")
                    .WithMany()
                    .HasForeignKey("AssignedToUserId")
                    .OnDelete(DeleteBehavior.SetNull);

                b.HasOne("Arxis.Domain.Entities.Project", "Project")
                    .WithMany()
                    .HasForeignKey("ProjectId")
                    .OnDelete(DeleteBehavior.Cascade)
                    .IsRequired();

                b.HasOne("Arxis.Domain.Entities.User", "ReportedByUser")
                    .WithMany()
                    .HasForeignKey("ReportedByUserId")
                    .OnDelete(DeleteBehavior.SetNull);

                b.HasOne("Arxis.Domain.Entities.User", "RespondedByUser")
                    .WithMany()
                    .HasForeignKey("RespondedByUserId")
                    .OnDelete(DeleteBehavior.SetNull);

                b.HasOne("Arxis.Domain.Entities.WorkTask", "WorkTask")
                    .WithMany()
                    .HasForeignKey("WorkTaskId")
                    .OnDelete(DeleteBehavior.SetNull);

                b.Navigation("AssignedToUser");

                b.Navigation("Project");

                b.Navigation("ReportedByUser");

                b.Navigation("RespondedByUser");

                b.Navigation("WorkTask");
            });

        modelBuilder.Entity("Arxis.Domain.Entities.IssueAttachment", b =>
            {
                b.HasOne("Arxis.Domain.Entities.IssueComment", "Comment")
                    .WithMany("Attachments")
                    .HasForeignKey("CommentId")
                    .OnDelete(DeleteBehavior.Cascade);

                b.HasOne("Arxis.Domain.Entities.Issue", "Issue")
                    .WithMany("Attachments")
                    .HasForeignKey("IssueId")
                    .OnDelete(DeleteBehavior.Cascade)
                    .IsRequired();

                b.HasOne("Arxis.Domain.Entities.User", "UploadedByUser")
                    .WithMany()
                    .HasForeignKey("UploadedByUserId")
                    .OnDelete(DeleteBehavior.SetNull);

                b.Navigation("Comment");

                b.Navigation("Issue");

                b.Navigation("UploadedByUser");
            });

        modelBuilder.Entity("Arxis.Domain.Entities.IssueComment", b =>
            {
                b.HasOne("Arxis.Domain.Entities.User", "Author")
                    .WithMany()
                    .HasForeignKey("AuthorId")
                    .OnDelete(DeleteBehavior.SetNull);

                b.HasOne("Arxis.Domain.Entities.Issue", "Issue")
                    .WithMany("Comments")
                    .HasForeignKey("IssueId")
                    .OnDelete(DeleteBehavior.Cascade)
                    .IsRequired();

                b.Navigation("Author");

                b.Navigation("Issue");
            });

        modelBuilder.Entity("Arxis.Domain.Entities.IssueLink", b =>
            {
                b.HasOne("Arxis.Domain.Entities.Issue", "Issue")
                    .WithMany("Links")
                    .HasForeignKey("IssueId")
                    .OnDelete(DeleteBehavior.Cascade)
                    .IsRequired();

                b.Navigation("Issue");
            });

        modelBuilder.Entity("Arxis.Domain.Entities.Project", b =>
            {
                b.Navigation("ProjectUsers");
            });

        modelBuilder.Entity("Arxis.Domain.Entities.User", b =>
            {
                b.Navigation("ProjectUsers");
            });

        modelBuilder.Entity("Arxis.Domain.Entities.Issue", b =>
            {
                b.Navigation("Attachments");

                b.Navigation("Comments");

                b.Navigation("Links");
            });

        modelBuilder.Entity("Arxis.Domain.Entities.IssueComment", b =>
            {
                b.Navigation("Attachments");
            });
#pragma warning restore 612, 618
    }
}
